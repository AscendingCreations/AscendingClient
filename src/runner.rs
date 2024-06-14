use crate::{
    content::*, data_types::*, database::*, systems::*, widget::*, Action,
};
use backtrace::Backtrace;
use camera::{
    controls::{Controls, FlatControls, FlatSettings},
    Projection,
};
use cosmic_text::{Attrs, Metrics};
use graphics::*;
use hecs::World;
use input::{Axis, Bindings, FrameTime, InputHandler, Key};
use log::{error, info, warn, LevelFilter, Metadata, Record};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;
use std::{collections::HashMap, env, num::NonZeroUsize};
use std::{
    fs::{self, File},
    io::{prelude::*, Read, Write},
    iter, panic,
    sync::Arc,
    time::{Duration, Instant},
};
use wgpu::{Backends, Dx12Compiler, InstanceDescriptor, InstanceFlags};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::NamedKey,
    platform::windows::WindowAttributesExtWindows,
    window::{WindowAttributes, WindowButtons},
};

#[allow(clippy::large_enum_variant)]
pub enum Runner {
    Loading,
    Ready {
        systems: SystemHolder,
        content: Content,
        world: World,
        graphics: State<FlatControls>,
        alert: Alert,
        tooltip: Tooltip,
        socket: Socket,
        router: PacketRouter,
        buffertask: BufferTask,
        input_handler: InputHandler<Action, Axis>,
        frame_time: FrameTime,
        time: f32,
        reconnect_time: f32,
        reset_timer: f32,
        fps: u32,
        start_ping: bool,
        reset_status: bool,
        loop_timer: LoopTimer,
        mouse_pos: PhysicalPosition<f64>,
        mouse_press: bool,
    },
}

impl winit::application::ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Self::Loading = self {
            // Load config
            let config = Config::read_config("settings.toml");

            info!("loading initiation");
            let win_attrs = WindowAttributes::default()
                .with_active(false)
                .with_visible(false)
                .with_inner_size(PhysicalSize::new(800, 600))
                .with_title("Client")
                .with_enabled_buttons({
                    let mut buttons = WindowButtons::all();
                    buttons.remove(WindowButtons::MAXIMIZE);
                    buttons
                });

            // Builds the Windows that will be rendered too.
            let window = Arc::new(
                event_loop.create_window(win_attrs).expect("Create window"),
            );

            info!("after window initiation");

            let backend = config.append_graphic_backend();

            // Generates an Instance for WGPU. Sets WGPU to be allowed on all possible supported backends
            // These are DX12, DX11, Vulkan, Metal and Gles. if none of these work on a system they cant
            // play the game basically.
            let instance = wgpu::Instance::new(InstanceDescriptor {
                backends: backend,
                flags: InstanceFlags::empty(),
                dx12_shader_compiler: Dx12Compiler::default(),
                gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
            });

            info!("after wgpu instance initiation");

            // This is used to ensure the GPU can load the correct.
            let compatible_surface =
                instance.create_surface(window.clone()).unwrap();

            info!("after compatible initiation");
            print!("{:?}", &compatible_surface);
            // This creates the Window Struct and Device struct that holds all the rendering information
            // we need to render to the screen. Window holds most of the window information including
            // the surface type. device includes the queue and GPU device for rendering.
            // This then adds gpu_window and gpu_device and creates our renderer type. for easy passing of window, device and font system.
            let mut renderer =
                futures::executor::block_on(instance.create_device(
                    window,
                    //used to find adapters
                    AdapterOptions {
                        allowed_backends: Backends::all(),
                        power: config.power_settings.parse_enum(),
                        compatible_surface: Some(compatible_surface),
                    },
                    // used to deturmine which adapters support our special limits or features for our backends.
                    &wgpu::DeviceDescriptor {
                        required_features: wgpu::Features::default(),
                        required_limits: wgpu::Limits::default(),
                        label: None,
                    },
                    None,
                    // How we are presenting the screen which causes it to either clip to a FPS limit or be unlimited.
                    config.present_mode.parse_enum(),
                ))
                .unwrap();

            info!("after renderer initiation");
            // we print the GPU it decided to use here for testing purposes.
            println!("{:?}", renderer.adapter().get_info());

            // We generate Texture atlases to use with out types.
            let mut atlases: Vec<AtlasSet> = iter::from_fn(|| {
                Some(AtlasSet::new(
                    &mut renderer,
                    wgpu::TextureFormat::Rgba8UnormSrgb,
                    true,
                ))
            })
            .take(4)
            .collect();

            // we generate the Text atlas seperatly since it contains a special texture that only has the red color to it.
            // and another for emojicons.
            let text_atlas = TextAtlas::new(&mut renderer).unwrap();

            let mut audio = Audio::new(0.15).unwrap();

            // get the screen size.
            let size = renderer.size();
            let mat = Mat4::from_translation(Vec3 {
                x: 40.0,
                y: 0.0,
                z: 0.0,
            });

            // get the Scale factor the pc currently is using for upscaling or downscaling the rendering.
            let scale = renderer
                .window()
                .current_monitor()
                .unwrap()
                .scale_factor()
                .clamp(1.0, 1.5);

            // Load textures image
            let resource =
                TextureAllocation::new(&mut atlases, &renderer).unwrap();

            let volume = config.sfx_volume as f32 * 0.01;
            audio.set_effect_volume(volume);
            let volume = config.bgm_volume as f32 * 0.01;
            audio.set_music_volume(volume);

            let database_holder = DatabaseHolder {
                item: load_items().unwrap(),
                shop: load_shops().unwrap(),
                npc: load_npcs().unwrap(),
                mapdata: SlotMap::with_key(),
                mappos_key: HashMap::default(),
                map_cache: LruCache::new(NonZeroUsize::new(64).unwrap()),
            };

            // Compile all rendering data in one type for quick access and passing
            let mut systems = SystemHolder {
                gfx: GfxCollection::new(),
                renderer,
                size,
                scale,
                resource,
                fade: Fade::new(),
                map_fade: MapFade::new(),
                config,
                base: database_holder,
                audio,
                caret: TextCaret {
                    visible: false,
                    index: None,
                    timer: 0.0,
                },
                try_once: true,
                fps: GfxType::None,
            };

            systems.fade.init_setup(
                &mut systems.renderer,
                &mut systems.gfx,
                &systems.size,
            );
            systems.map_fade.init_setup(
                &mut systems.renderer,
                &mut systems.gfx,
                &systems.size,
            );

            // We establish the different renderers here to load their data up to use them.
            let text_renderer = TextRenderer::new(&systems.renderer).unwrap();
            let image_renderer = ImageRenderer::new(&systems.renderer).unwrap();
            let map_renderer =
                MapRenderer::new(&mut systems.renderer, 81).unwrap();
            let light_renderer =
                LightRenderer::new(&mut systems.renderer).unwrap();
            let ui_renderer = RectRenderer::new(&systems.renderer).unwrap();

            let mut world = World::new();
            let buffertask = BufferTask::new();

            // Initiate Game Content
            let mut content = Content::new(&mut world, &mut systems).unwrap();

            let alert = Alert::new();

            let tooltip = Tooltip::new(&mut systems);

            let mut socket = Socket::new(&systems.config).unwrap();
            let router = PacketRouter::init();
            socket.register().unwrap();
            content.menu_content.set_status_offline(&mut systems);

            // setup our system which includes Camera and projection as well as our controls.
            // for the camera.
            let system = System::new(
                &mut systems.renderer,
                Projection::Orthographic {
                    left: 0.0,
                    right: systems.size.width,
                    bottom: 0.0,
                    top: systems.size.height,
                    near: 1.0,
                    far: -100.0,
                },
                FlatControls::new(FlatSettings { zoom: 1.0 }),
                [systems.size.width, systems.size.height],
                mat,
                1.5,
            );

            // create a Text rendering object.
            let text_scale = systems.scale as f32;
            let txt_pos = Vec2::new(
                systems.size.width - (150.0 * text_scale).floor(),
                systems.size.height - (25.0 * text_scale).floor(),
            );
            let txt = create_label(
                &mut systems,
                Vec3::new(txt_pos.x, txt_pos.y, 0.0),
                (Vec2::new(150.0, 20.0) * text_scale).floor(),
                Bounds::new(
                    txt_pos.x,
                    txt_pos.y,
                    txt_pos.x + (150.0 * text_scale).floor(),
                    txt_pos.y + (20.0 * text_scale).floor(),
                ),
                Color::rgba(255, 255, 255, 255),
            );
            let text =
                systems.gfx.add_text(txt, 5, "FPS", systems.config.show_fps);
            systems.fps = text;

            // Allow the window to be seen. hiding it then making visible speeds up
            // load times.
            systems.renderer.window().set_visible(true);

            // add everything into our convience type for quicker access and passing.
            let graphics = State {
                system,
                image_atlas: atlases.remove(0),
                text_atlas,
                map_atlas: atlases.remove(0),
                ui_atlas: atlases.remove(0),
                image_renderer,
                text_renderer,
                map_renderer,
                light_renderer,
                ui_renderer,
            };

            // Create the mouse/keyboard bindings for our stuff.
            let mut bindings = Bindings::<Action, Axis>::new();
            bindings
                .insert_action(Action::Quit, vec![Key::Character('q').into()]);

            // set bindings and create our own input handler.
            // Increase the milli second to higher numbers if you need to support accessability for
            // slower clicking users. can have presets.
            let input_handler =
                InputHandler::new(bindings, Duration::from_millis(180));

            systems.audio.set_music("./audio/caves.ogg").unwrap();
            content.game_content.current_music = "caves.ogg".to_string();

            systems.renderer.window().set_visible(true);

            *self = Self::Ready {
                content,
                systems,
                world,
                graphics,
                input_handler,
                alert,
                tooltip,
                socket,
                router,
                buffertask,
                frame_time: FrameTime::new(),
                time: 0.0f32,
                reconnect_time: 0.0f32,
                reset_timer: 0.0f32,
                fps: 0u32,
                start_ping: true,
                reset_status: false,
                loop_timer: LoopTimer::default(),
                mouse_pos: PhysicalPosition::new(0.0, 0.0),
                mouse_press: false,
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Self::Ready {
            content,
            systems,
            world,
            graphics,
            input_handler,
            alert,
            tooltip,
            socket,
            router,
            buffertask,
            frame_time,
            time,
            reconnect_time,
            reset_timer,
            fps,
            start_ping,
            reset_status,
            loop_timer,
            mouse_pos,
            mouse_press,
        } = self
        {
            let frame_time_start = MyInstant::now();

            frame_time.update();
            let seconds = frame_time.seconds();

            if window_id == systems.renderer.window().id() {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("The close button was pressed; stopping");
                        event_loop.exit();
                        return;
                    }
                    WindowEvent::Focused(focused) => {
                        if !focused {
                            content.game_content.keyinput.iter_mut().for_each(
                                |key| {
                                    *key = false;
                                },
                            )
                        }
                    }
                    _ => {}
                }
            }

            // update our inputs.
            input_handler.window_updates(
                systems.renderer.window(),
                &event,
                1.0,
            );

            for input in input_handler.events() {
                match input {
                    input::InputEvent::KeyInput { key, pressed, .. } => {
                        handle_key_input(
                            world, systems, socket, content, alert, key,
                            *pressed,
                        )
                        .unwrap();
                    }
                    input::InputEvent::MouseButton { button, pressed } => {
                        if *button == MouseButton::Left {
                            if *pressed {
                                handle_mouse_input(
                                    world,
                                    systems,
                                    socket,
                                    event_loop,
                                    MouseInputType::MouseLeftDown,
                                    &Vec2::new(
                                        mouse_pos.x as f32,
                                        mouse_pos.y as f32,
                                    ),
                                    content,
                                    alert,
                                    tooltip,
                                )
                                .unwrap();
                                *mouse_press = true;
                            } else if *mouse_press {
                                handle_mouse_input(
                                    world,
                                    systems,
                                    socket,
                                    event_loop,
                                    MouseInputType::MouseRelease,
                                    &Vec2::new(
                                        mouse_pos.x as f32,
                                        mouse_pos.y as f32,
                                    ),
                                    content,
                                    alert,
                                    tooltip,
                                )
                                .unwrap();
                                *mouse_press = false;
                            }
                        }
                    }
                    input::InputEvent::MousePosition => {
                        if let Some(position) =
                            input_handler.physical_mouse_position()
                        {
                            *mouse_pos = position;

                            if *mouse_press {
                                handle_mouse_input(
                                    world,
                                    systems,
                                    socket,
                                    event_loop,
                                    MouseInputType::MouseLeftDownMove,
                                    &Vec2::new(
                                        position.x as f32,
                                        position.y as f32,
                                    ),
                                    content,
                                    alert,
                                    tooltip,
                                )
                                .unwrap();
                            } else {
                                handle_mouse_input(
                                    world,
                                    systems,
                                    socket,
                                    event_loop,
                                    MouseInputType::MouseMove,
                                    &Vec2::new(
                                        position.x as f32,
                                        position.y as f32,
                                    ),
                                    content,
                                    alert,
                                    tooltip,
                                )
                                .unwrap();
                            }
                        }
                    }
                    input::InputEvent::MouseButtonAction(action) => {
                        if let input::MouseButtonAction::Double(_) = action {
                            handle_mouse_input(
                                world,
                                systems,
                                socket,
                                event_loop,
                                MouseInputType::MouseDoubleLeftDown,
                                &Vec2::new(
                                    mouse_pos.x as f32,
                                    mouse_pos.y as f32,
                                ),
                                content,
                                alert,
                                tooltip,
                            )
                            .unwrap();
                        }
                    }
                    _ => {}
                }
            }

            // update our renderer based on events here
            if !systems.renderer.update(&event).unwrap() {
                return;
            }

            // get the current window size so we can see if we need to resize the renderer.
            let new_size = systems.renderer.size();

            if systems.size != new_size {
                systems.size = new_size;

                // Reset screen size for the Surface here.
                graphics.system.set_projection(Projection::Orthographic {
                    left: 0.0,
                    right: new_size.width,
                    bottom: 0.0,
                    top: new_size.height,
                    near: 1.0,
                    far: -100.0,
                });

                systems.renderer.update_depth_texture();
            }

            if let Some(gfx_index) = systems.caret.index {
                if systems.caret.timer <= seconds {
                    systems.caret.visible = !systems.caret.visible;
                    systems.caret.timer = seconds + 0.35;
                    systems.gfx.set_visible(&gfx_index, systems.caret.visible);
                }
            }

            // Game Loop
            game_loop(
                socket, world, systems, content, buffertask, seconds,
                loop_timer,
            )
            .unwrap();
            if systems.fade.fade_logic(&mut systems.gfx, seconds) {
                fade_end(systems, world, content, socket, buffertask).unwrap();
            }
            if systems.map_fade.fade_logic(&mut systems.gfx, seconds) {
                map_fade_end(systems, world, content);
            }
            tooltip.handle_tooltip_logic(systems, seconds);

            // update our systems data to the gpu. this is the Camera in the shaders.
            graphics.system.update(&systems.renderer, frame_time);

            // update our systems data to the gpu. this is the Screen in the shaders.
            graphics.system.update_screen(
                &systems.renderer,
                [new_size.width, new_size.height],
            );

            // This adds the Image data to the Buffer for rendering.
            add_image_to_buffer(systems, graphics);

            // this cycles all the Image's in the Image buffer by first putting them in rendering order
            // and then uploading them to the GPU if they have moved or changed in any way. clears the
            // Image buffer for the next render pass. Image buffer only holds the ID's and Sortign info
            // of the finalized Indicies of each Image.
            graphics.image_renderer.finalize(&mut systems.renderer);
            graphics.text_renderer.finalize(&mut systems.renderer);
            graphics.map_renderer.finalize(&mut systems.renderer);
            graphics.light_renderer.finalize(&mut systems.renderer);
            graphics.ui_renderer.finalize(&mut systems.renderer);

            // Start encoding commands. this stores all the rendering calls for execution when
            // finish is called.
            let mut encoder = systems.renderer.device().create_command_encoder(
                &wgpu::CommandEncoderDescriptor {
                    label: Some("command encoder"),
                },
            );

            // Run the render pass. for the games renderer
            graphics.render(&systems.renderer, &mut encoder);

            // Submit our command queue. for it to upload all the changes that were made.
            // Also tells the system to begin running the commands on the GPU.
            systems
                .renderer
                .queue()
                .submit(std::iter::once(encoder.finish()));

            let disconnect = match poll_events(socket) {
                Ok(d) => d,
                Err(e) => {
                    error!("Poll event error: {:?}", e);
                    true
                }
            };

            if disconnect || socket.client.state == ClientState::Closed {
                if *reconnect_time < seconds {
                    if content.content_type == ContentType::Game {
                        alert.show_alert(
                            systems,
                            AlertType::Inform,
                            "You have been disconnected".into(),
                            "Alert Message".into(),
                            250,
                            AlertIndex::None,
                            false,
                        );

                        content
                            .switch_content(world, systems, ContentType::Menu)
                            .unwrap();
                    }

                    *start_ping = true;
                    socket.reconnect().unwrap();
                    socket.register().unwrap();
                }
                content.menu_content.set_status_offline(systems);
                *reconnect_time = seconds + 1.0;
            } else if *reset_timer < seconds && *reset_status {
                *reset_status = false;
                content.menu_content.set_status_online(systems);
            }

            if *start_ping {
                *start_ping = false;
                *reset_status = true;
                *reset_timer = seconds + 3.0;
                send_ping(socket).unwrap();
            }

            process_packets(
                socket, router, world, systems, content, alert, seconds,
                buffertask,
            )
            .unwrap();

            buffertask.process_buffer(systems, content).unwrap();

            if *time < seconds {
                systems.gfx.set_rich_text(
                    &mut systems.renderer,
                    &systems.fps,
                    [
                        (
                            "FPS: ",
                            Attrs::new().color(Color::rgba(200, 100, 100, 255)),
                        ),
                        (
                            &format!("{fps}"),
                            Attrs::new().color(Color::rgba(255, 255, 255, 255)),
                        ),
                    ],
                );
                *fps = 0u32;
                *time = seconds + 1.0;
            }

            systems.audio.update_effects();

            *fps += 1;

            systems.renderer.window().pre_present_notify();
            systems.renderer.present().unwrap();

            // These clear the Last used image tags.
            //Can be used later to auto unload things not used anymore if ram/gpu ram becomes a issue.
            if *fps == 1 {
                graphics.image_atlas.trim();
            }

            if *fps == 2 {
                graphics.map_atlas.trim();
            }

            if *fps == 3 {
                graphics.text_atlas.trim();
            }

            if *fps == 4 {
                systems.renderer.font_sys.shape_run_cache.trim(1024);
            }

            let frame_time_end = MyInstant::now();

            if systems.config.show_frame_loop {
                let elapse_time = frame_time_end
                    .duration_since(frame_time_start.0)
                    .as_millis() as u64;

                let count =
                    content.game_content.interface.frame_loop_collection.len();
                if count > 0 {
                    let sum: u64 = content
                        .game_content
                        .interface
                        .frame_loop_collection
                        .iter()
                        .sum();
                    if sum > 0 {
                        let average: u64 = sum / count as u64;
                        systems.gfx.set_text(
                            &mut systems.renderer,
                            &content.game_content.interface.frame_loop,
                            &format!("Frame Jitter: {:?}", average),
                        );
                    }
                    if count >= 20 {
                        content
                            .game_content
                            .interface
                            .frame_loop_collection
                            .pop_back();
                    }
                }
                content
                    .game_content
                    .interface
                    .frame_loop_collection
                    .push_front(elapse_time);
            }
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        if let Self::Ready {
            content: _,
            systems,
            world: _,
            graphics: _,
            router: _,
            buffertask: _,
            input_handler,
            alert: _,
            tooltip: _,
            socket: _,
            frame_time: _,
            time: _,
            reconnect_time: _,
            reset_timer: _,
            fps: _,
            start_ping: _,
            reset_status: _,
            loop_timer: _,
            mouse_pos: _,
            mouse_press: _,
        } = self
        {
            input_handler.device_updates(systems.renderer.window(), &event);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Self::Ready {
            content: _,
            systems,
            world: _,
            graphics: _,
            router: _,
            buffertask: _,
            input_handler: _,
            alert: _,
            tooltip: _,
            socket: _,
            frame_time: _,
            time: _,
            reconnect_time: _,
            reset_timer: _,
            fps: _,
            start_ping: _,
            reset_status: _,
            loop_timer: _,
            mouse_pos: _,
            mouse_press: _,
        } = self
        {
            systems.renderer.window().request_redraw();
        }
    }
}
