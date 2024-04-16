#![feature(error_generic_member_access)]
#![allow(
    dead_code,
    clippy::collapsible_match,
    unused_imports,
    clippy::too_many_arguments
)]
use backtrace::Backtrace;
use camera::{
    controls::{Controls, FlatControls, FlatSettings},
    Projection,
};
use cosmic_text::{Attrs, Metrics};
use graphics::*;
use hecs::World;
use input::{Bindings, FrameTime, InputHandler, Key};
use log::{error, info, warn, Level, LevelFilter, Metadata, Record};
use serde::{Deserialize, Serialize};
use std::env;
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
    event_loop::{ControlFlow, EventLoop},
    keyboard::NamedKey,
    window::{WindowBuilder, WindowButtons},
};

mod alert;
mod audio;
mod buffer;
pub mod config;
mod content;
mod data_types;
mod database;
mod error;
mod gfx_collection;
mod inputs;
mod logic;
mod mainloop;
mod renderer;
mod resource;
mod socket;
mod time_ext;
mod values;
mod widget;

use alert::*;
use audio::*;
use buffer::*;
pub use config::*;
use content::*;
use database::*;
pub use error::*;
use gfx_collection::*;
use inputs::*;
use logic::*;
use mainloop::*;
use renderer::*;
use resource::*;
use socket::*;
use values::*;
use widget::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum Action {
    Quit,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum Axis {
    Forward,
    Sideward,
    Yaw,
    Pitch,
}

enum MouseEvent {
    None,
    Click,
    Release,
}

// creates a static global logger type for setting the logger
static MY_LOGGER: MyLogger = MyLogger(Level::Debug);
pub static APP_MAJOR: u16 = 1;
pub static APP_MINOR: u16 = 1;
pub static APP_REV: u16 = 1;

struct MyLogger(pub Level);

impl log::Log for MyLogger {
    // checks if it can log these types of events.
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.0
    }

    // This logs to a panic file. This is so we can see
    // Errors and such if a program crashes in full render mode.
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("{} - {}\n", record.level(), record.args());
            println!("{}", &msg);

            let mut file = match File::options()
                .append(true)
                .create(true)
                .open("paniclog.txt")
            {
                Ok(v) => v,
                Err(_) => return,
            };

            let _ = file.write(msg.as_bytes());
        }
    }
    fn flush(&self) {}
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create logger to output to a File
    log::set_logger(&MY_LOGGER).unwrap();
    // Set the Max level we accept logging to the file for.
    log::set_max_level(LevelFilter::Info);

    //Comment this out if you do not want a backtrace on error to show.
    //env::set_var("RUST_BACKTRACE", "1");

    // This allows us to take control of panic!() so we can send it to a file via the logger.
    panic::set_hook(Box::new(|panic_info| {
        let bt = Backtrace::new();

        error!("PANIC: {}, BACKTRACE: {:?}", panic_info, bt);
    }));

    // Starts an event gathering type for the window.
    let event_loop = EventLoop::new()?;

    // Builds the Windows that will be rendered too.
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Client")
            .with_inner_size(PhysicalSize::new(
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            ))
            .with_visible(false)
            .with_enabled_buttons({
                let mut buttons = WindowButtons::all();
                buttons.remove(WindowButtons::MAXIMIZE);
                buttons
            })
            .with_resizable(false)
            .build(&event_loop)
            .unwrap(),
    );

    // Generates an Instance for WGPU. Sets WGPU to be allowed on all possible supported backends
    // These are DX12, DX11, Vulkan, Metal and Gles. if none of these work on a system they cant
    // play the game basically.
    let instance = wgpu::Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        flags: InstanceFlags::empty(),
        dx12_shader_compiler: Dx12Compiler::default(),
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    });

    // This is used to ensure the GPU can load the correct.
    let compatible_surface = instance.create_surface(window.clone()).unwrap();

    // This creates the Window Struct and Device struct that holds all the rendering information
    // we need to render to the screen. Window holds most of the window information including
    // the surface type. device includes the queue and GPU device for rendering.
    // This then adds gpu_window and gpu_device and creates our renderer type. for easy passing of window, device and font system.
    let mut renderer = instance
        .create_device(
            window,
            //used to find adapters
            AdapterOptions {
                allowed_backends: Backends::all(),
                power: AdapterPowerSettings::HighPower,
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
            wgpu::PresentMode::AutoVsync,
        )
        .await
        .unwrap();

    // get the screen size.
    let size = renderer.size();

    // get the Scale factor the pc currently is using for upscaling or downscaling the rendering.
    let scale = renderer.window().current_monitor().unwrap().scale_factor();

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

    let mut audio = Audio::new(0.15)?;

    // Load textures image
    let resource = TextureAllocation::new(&mut atlases, &renderer).unwrap();

    // Load config
    let config = Config::read_config("settings.toml");

    let volume = config.sfx_volume as f32 * 0.01;
    audio.set_effect_volume(volume);
    let volume = config.bgm_volume as f32 * 0.01;
    audio.set_music_volume(volume);

    let database_holder = DatabaseHolder {
        item: get_item()?,
        shop: get_shop()?,
        npc: get_npc()?,
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
    let map_renderer = MapRenderer::new(&mut systems.renderer, 81).unwrap();
    let ui_renderer = RectRenderer::new(&systems.renderer).unwrap();

    let mut world = World::new();
    let mut buffertask = BufferTask::new();

    // Initiate Game Content
    let mut content = Content::new(&mut world, &mut systems)?;

    let mut alert = Alert::new();

    let mut tooltip = Tooltip::new(&mut systems);

    let mut socket = Socket::new(&systems.config).unwrap();
    let router = PacketRouter::init();
    socket.register().expect("Failed to register socket");
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
    );

    // create a Text rendering object.
    let txt_pos =
        Vec2::new(systems.size.width - 80.0, systems.size.height - 25.0);
    let txt = create_label(
        &mut systems,
        Vec3::new(txt_pos.x, txt_pos.y, 0.0),
        Vec2::new(100.0, 20.0),
        Bounds::new(txt_pos.x, txt_pos.y, txt_pos.x + 100.0, txt_pos.y + 20.0),
        Color::rgba(255, 255, 255, 255),
    );
    let text = systems.gfx.add_text(txt, 4);

    // Allow the window to be seen. hiding it then making visible speeds up
    // load times.
    systems.renderer.window().set_visible(true);

    // add everything into our convience type for quicker access and passing.
    let mut graphics = State {
        system,
        image_atlas: atlases.remove(0),
        text_atlas,
        map_atlas: atlases.remove(0),
        ui_atlas: atlases.remove(0),
        image_renderer,
        text_renderer,
        map_renderer,
        ui_renderer,
    };

    // Create the mouse/keyboard bindings for our stuff.
    let mut bindings = Bindings::<Action, Axis>::new();
    bindings.insert_action(Action::Quit, vec![Key::Character('q').into()]);

    // set bindings and create our own input handler.
    // Increase the milli second to higher numbers if you need to support accessability for
    // slower clicking users. can have presets.
    let mut input_handler =
        InputHandler::new(bindings, Duration::from_millis(180));

    let mut frame_time = FrameTime::new();
    let mut time = 0.0f32;
    let mut reconnect_time = 0.0f32;
    let mut reset_timer = 0.0f32;
    let mut fps = 0u32;
    let mut start_ping = true;
    let mut reset_status = true;
    let fps_label_color = Attrs::new().color(Color::rgba(200, 100, 100, 255));
    let fps_number_color = Attrs::new().color(Color::rgba(255, 255, 255, 255));
    let mut loop_timer = LoopTimer::default();

    let mut mouse_pos: PhysicalPosition<f64> = PhysicalPosition::new(0.0, 0.0);
    let mut mouse_press: bool = false;

    #[allow(deprecated)]
    event_loop.run(move |event, elwt| {
        frame_time.update();
        let seconds = frame_time.seconds();

        // we check for the first batch of events to ensure we dont need to stop rendering here first.
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
                ..
            } if window_id == systems.renderer.window().id() => {
                if let WindowEvent::CloseRequested = event {
                    elwt.exit();
                }
            }
            Event::AboutToWait => {
                systems.renderer.window().request_redraw();
            }
            _ => {}
        }

        // update our inputs.
        input_handler.update(systems.renderer.window(), &event, 1.0);

        for input in input_handler.events() {
            match input {
                input::InputEvent::KeyInput { key, pressed, .. } => {
                    handle_key_input(
                        &mut world,
                        &mut systems,
                        &mut socket,
                        &mut content,
                        &mut alert,
                        key,
                        *pressed,
                    )
                    .unwrap();
                }
                input::InputEvent::MouseButton { button, pressed } => {
                    if *button == MouseButton::Left {
                        if *pressed {
                            handle_mouse_input(
                                &mut world,
                                &mut systems,
                                &mut socket,
                                elwt,
                                MouseInputType::MouseLeftDown,
                                &Vec2::new(
                                    mouse_pos.x as f32,
                                    mouse_pos.y as f32,
                                ),
                                &mut content,
                                &mut alert,
                                &mut tooltip,
                            )
                            .unwrap();
                            mouse_press = true;
                        } else if mouse_press {
                            handle_mouse_input(
                                &mut world,
                                &mut systems,
                                &mut socket,
                                elwt,
                                MouseInputType::MouseRelease,
                                &Vec2::new(
                                    mouse_pos.x as f32,
                                    mouse_pos.y as f32,
                                ),
                                &mut content,
                                &mut alert,
                                &mut tooltip,
                            )
                            .unwrap();
                            mouse_press = false;
                        }
                    }
                }
                input::InputEvent::MousePosition => {
                    if let Some(position) =
                        input_handler.physical_mouse_position()
                    {
                        mouse_pos = position;

                        if mouse_press {
                            handle_mouse_input(
                                &mut world,
                                &mut systems,
                                &mut socket,
                                elwt,
                                MouseInputType::MouseLeftDownMove,
                                &Vec2::new(
                                    position.x as f32,
                                    position.y as f32,
                                ),
                                &mut content,
                                &mut alert,
                                &mut tooltip,
                            )
                            .unwrap();
                        } else {
                            handle_mouse_input(
                                &mut world,
                                &mut systems,
                                &mut socket,
                                elwt,
                                MouseInputType::MouseMove,
                                &Vec2::new(
                                    position.x as f32,
                                    position.y as f32,
                                ),
                                &mut content,
                                &mut alert,
                                &mut tooltip,
                            )
                            .unwrap();
                        }
                    }
                }
                input::InputEvent::MouseButtonAction(action) => {
                    if let input::MouseButtonAction::Double(_) = action {
                        handle_mouse_input(
                            &mut world,
                            &mut systems,
                            &mut socket,
                            elwt,
                            MouseInputType::MouseDoubleLeftDown,
                            &Vec2::new(mouse_pos.x as f32, mouse_pos.y as f32),
                            &mut content,
                            &mut alert,
                            &mut tooltip,
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
                systems.gfx.set_visible(gfx_index, systems.caret.visible);
            }
        }

        // Game Loop
        game_loop(
            &mut socket,
            &mut world,
            &mut systems,
            &mut content,
            &mut buffertask,
            seconds,
            &mut loop_timer,
        )
        .unwrap();
        if systems.fade.fade_logic(&mut systems.gfx, seconds) {
            fade_end(&mut systems, &mut world, &mut content, &mut socket)
                .unwrap();
        }
        if systems.map_fade.fade_logic(&mut systems.gfx, seconds) {
            map_fade_end(&mut systems, &mut world, &mut content);
        }
        tooltip.handle_tooltip_logic(&mut systems, seconds);

        // update our systems data to the gpu. this is the Camera in the shaders.
        graphics.system.update(&systems.renderer, &frame_time);

        // update our systems data to the gpu. this is the Screen in the shaders.
        graphics.system.update_screen(
            &systems.renderer,
            [new_size.width, new_size.height],
        );

        // This adds the Image data to the Buffer for rendering.
        add_image_to_buffer(&mut systems, &mut graphics);

        // this cycles all the Image's in the Image buffer by first putting them in rendering order
        // and then uploading them to the GPU if they have moved or changed in any way. clears the
        // Image buffer for the next render pass. Image buffer only holds the ID's and Sortign info
        // of the finalized Indicies of each Image.
        graphics.image_renderer.finalize(&mut systems.renderer);
        graphics.text_renderer.finalize(&mut systems.renderer);
        graphics.map_renderer.finalize(&mut systems.renderer);
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

        let disconnect = match poll_events(&mut socket) {
            Ok(d) => d,
            Err(e) => {
                println!("Poll event error: {:?}", e);
                true
            }
        };

        if disconnect || socket.client.state == ClientState::Closed {
            if reconnect_time < seconds {
                if content.content_type == ContentType::Game {
                    alert.show_alert(
                        &mut systems,
                        AlertType::Inform,
                        "You have been disconnected".into(),
                        "Alert Message".into(),
                        250,
                        AlertIndex::None,
                        false,
                    );

                    content
                        .switch_content(
                            &mut world,
                            &mut systems,
                            ContentType::Menu,
                        )
                        .unwrap();
                }

                start_ping = true;
                socket.reconnect().unwrap();
                socket.register().unwrap();
            }
            content.menu_content.set_status_offline(&mut systems);
            reconnect_time = seconds + 1.0;
        } else if reset_timer < seconds && reset_status {
            reset_status = false;
            content.menu_content.set_status_online(&mut systems);
        }

        if start_ping {
            start_ping = false;
            reset_status = true;
            reset_timer = seconds + 3.0;
            println!("ping sent");
            send_ping(&mut socket).unwrap();
        }

        process_packets(
            &mut socket,
            &router,
            &mut world,
            &mut systems,
            &mut content,
            &mut alert,
            seconds,
            &mut buffertask,
        )
        .unwrap();

        buffertask
            .process_buffer(&mut systems, &mut content)
            .unwrap();

        if time < seconds {
            systems.gfx.set_rich_text(
                &mut systems.renderer,
                text,
                [
                    ("FPS: ", fps_label_color),
                    (&format!("{fps}"), fps_number_color),
                ],
            );
            fps = 0u32;
            time = seconds + 1.0;
        }

        systems.audio.update_effects();

        fps += 1;

        systems.renderer.window().pre_present_notify();
        systems.renderer.present().unwrap();

        // These clear the Last used image tags.
        //Can be used later to auto unload things not used anymore if ram/gpu ram becomes a issue.
        if fps == 1 {
            graphics.image_atlas.trim();
            graphics.map_atlas.trim();
            graphics.text_atlas.trim();
            systems.renderer.font_sys.shape_run_cache.trim(1024);
        }
    })?;

    Ok(())
}
