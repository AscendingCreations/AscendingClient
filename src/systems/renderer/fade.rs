use camera::controls::FlatControls;
use graphics::*;

use winit::dpi::PhysicalSize;

use crate::{
    BufferTask, Entity, Position, Result, SystemHolder, World,
    content::*,
    data_types::*,
    systems::{Poller, State},
};

#[derive(Default)]
pub enum FadeData {
    #[default]
    None,
}

#[derive(Default)]
pub enum FadeType {
    #[default]
    In,
    Out,
}

pub const FADE_SWITCH_TO_GAME: usize = 1;
pub const FADE_SWITCH_TO_TITLE: usize = 2;

#[derive(Default)]
pub struct Fade {
    pub show: bool,
    pub f_image: GfxType,
    f_tmr: f32,
    pub f_alpha: isize,
    f_type: FadeType,
    f_end_index: usize,
    f_data: FadeData,
}

impl Fade {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_setup(
        &mut self,
        renderer: &mut GpuRenderer,
        gfx_collection: &mut GfxCollection,
        screen_size: &PhysicalSize<f32>,
    ) {
        let rect = Rect::new(
            renderer,
            Vec3::new(0.0, 0.0, ORDER_FADE),
            Vec2::new(screen_size.width, screen_size.height),
            Color::rgba(0, 0, 0, 0),
            0,
        );

        self.f_image = gfx_collection.add_rect(
            rect,
            4,
            "Fade Image",
            true,
            CameraView::SubView1,
        );
        self.show = false;
        self.f_tmr = 0.0;
        self.f_alpha = 0;
        self.f_end_index = 0;
    }

    pub fn fade_logic(
        &mut self,
        gfx_collection: &mut GfxCollection,
        seconds: f32,
    ) -> bool {
        if !self.show {
            return false;
        }

        let mut did_end = false;

        if self.f_tmr <= seconds {
            match self.f_type {
                FadeType::In => {
                    self.f_alpha = (self.f_alpha + 8).min(255);

                    if self.f_alpha >= 255 {
                        self.show = false;
                        did_end = true;
                    }
                }
                FadeType::Out => {
                    self.f_alpha = (self.f_alpha - 8).max(0);

                    if self.f_alpha <= 0 {
                        self.show = false;
                        did_end = true;
                    }
                }
            }

            gfx_collection.set_color(
                &self.f_image,
                Color::rgba(0, 0, 0, self.f_alpha as u8),
            );
            self.f_tmr = seconds + 0.03;
        }

        did_end
    }

    pub fn init_fade(
        &mut self,
        gfx_collection: &mut GfxCollection,
        fade_type: FadeType,
        fade_end_index: usize,
        fade_data: FadeData,
    ) {
        match fade_type {
            FadeType::In => {
                self.f_alpha = 0;
            }
            FadeType::Out => {
                self.f_alpha = 255;
            }
        }

        gfx_collection
            .set_color(&self.f_image, Color::rgba(0, 0, 0, self.f_alpha as u8));
        self.f_type = fade_type;
        self.f_end_index = fade_end_index;
        self.f_data = fade_data;
        self.show = true;
    }
}

#[derive(Default)]
pub struct MapFade {
    show: bool,
    f_image: GfxType,
    f_tmr: f32,
    pub f_alpha: isize,
    f_type: FadeType,
    f_end_index: usize,
    f_data: FadeData,
}

impl MapFade {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_setup(
        &mut self,
        renderer: &mut GpuRenderer,
        gfx_collection: &mut GfxCollection,
        screen_size: &PhysicalSize<f32>,
    ) {
        let rect = Rect::new(
            renderer,
            Vec3::new(0.0, 0.0, ORDER_MAP_FADE),
            Vec2::new(screen_size.width, screen_size.height),
            Color::rgba(0, 0, 0, 0),
            0,
        );

        self.f_image = gfx_collection.add_rect(
            rect,
            4,
            "Map Fade Image",
            true,
            CameraView::SubView1,
        );
        self.show = false;
        self.f_tmr = 0.0;
        self.f_alpha = 0;
        self.f_end_index = 0;
    }

    pub fn fade_logic(
        &mut self,
        gfx_collection: &mut GfxCollection,
        seconds: f32,
    ) -> bool {
        if !self.show {
            return false;
        }

        let mut did_end = false;

        if self.f_tmr <= seconds {
            match self.f_type {
                FadeType::In => {
                    self.f_alpha = (self.f_alpha + 8).min(255);

                    if self.f_alpha >= 255 {
                        self.show = false;
                        did_end = true;
                    }
                }
                FadeType::Out => {
                    self.f_alpha = (self.f_alpha - 8).max(0);

                    if self.f_alpha <= 0 {
                        self.show = false;
                        did_end = true;
                    }
                }
            }
            gfx_collection.set_color(
                &self.f_image,
                Color::rgba(0, 0, 0, self.f_alpha as u8),
            );
            self.f_tmr = seconds + 0.03;
        }

        did_end
    }

    pub fn force_fade(
        &mut self,
        gfx_collection: &mut GfxCollection,
        fade_type: FadeType,
    ) {
        match fade_type {
            FadeType::In => {
                self.f_alpha = 255;
            }
            FadeType::Out => {
                self.f_alpha = 0;
            }
        }

        gfx_collection
            .set_color(&self.f_image, Color::rgba(0, 0, 0, self.f_alpha as u8));
    }

    pub fn init_fade(
        &mut self,
        gfx_collection: &mut GfxCollection,
        fade_type: FadeType,
        fade_end_index: usize,
        fade_data: FadeData,
    ) {
        match fade_type {
            FadeType::In => {
                self.f_alpha = 0;
            }
            FadeType::Out => {
                self.f_alpha = 255;
            }
        }

        gfx_collection
            .set_color(&self.f_image, Color::rgba(0, 0, 0, self.f_alpha as u8));
        self.f_type = fade_type;
        self.f_end_index = fade_end_index;
        self.f_data = fade_data;
        self.show = true;
    }
}

pub fn fade_end(
    systems: &mut SystemHolder,
    graphics: &mut State<FlatControls>,
    world: &mut World,
    content: &mut Content,
    _socket: &mut Poller,
    buffer: &mut BufferTask,
) -> Result<()> {
    #[allow(clippy::single_match)]
    match systems.fade.f_end_index {
        FADE_SWITCH_TO_GAME => {
            content.switch_content(
                world,
                systems,
                &mut graphics.map_renderer,
                ContentType::Game,
            )?;

            let pos = if let Some(entity) = content.game_content.myentity {
                if let Some(Entity::Player(p_data)) = world.entities.get(entity)
                {
                    p_data.pos
                } else {
                    Position::default()
                }
            } else {
                Position::default()
            };

            content.game_content.init_map(
                systems,
                &mut graphics.map_renderer,
                pos.map,
                buffer,
                true,
            )?;
            content
                .game_content
                .init_finalized_data(world, systems, graphics)?;

            systems.fade.init_fade(
                &mut systems.gfx,
                FadeType::Out,
                0,
                FadeData::None,
            );
        }
        FADE_SWITCH_TO_TITLE => {
            content.switch_content(
                world,
                systems,
                &mut graphics.map_renderer,
                ContentType::Menu,
            )?;

            systems.fade.init_fade(
                &mut systems.gfx,
                FadeType::Out,
                0,
                FadeData::None,
            );
        }
        _ => {}
    }
    Ok(())
}

pub fn map_fade_end(
    systems: &mut SystemHolder,
    _world: &mut World,
    _content: &mut Content,
) {
    #[allow(clippy::single_match)]
    match systems.map_fade.f_end_index {
        1 => {
            systems.map_fade.init_fade(
                &mut systems.gfx,
                FadeType::Out,
                0,
                FadeData::None,
            );
        }
        _ => {}
    }
}
