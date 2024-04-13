use graphics::*;

use hecs::World;
use winit::dpi::PhysicalSize;

use crate::{
    content::*, gfx_collection::*, values::*, Result, Socket, SystemHolder,
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

#[derive(Default)]
pub struct Fade {
    show: bool,
    f_image: usize,
    f_tmr: f32,
    f_alpha: isize,
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
        let mut rect = Rect::new(renderer, 0);
        rect.set_size(Vec2::new(screen_size.width, screen_size.height))
            .set_position(Vec3::new(0.0, 0.0, ORDER_FADE))
            .set_color(Color::rgba(0, 0, 0, 0));
        self.f_image = gfx_collection.add_rect(rect, 4);

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
                self.f_image,
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
            .set_color(self.f_image, Color::rgba(0, 0, 0, self.f_alpha as u8));
        self.f_type = fade_type;
        self.f_end_index = fade_end_index;
        self.f_data = fade_data;
        self.show = true;
    }
}

#[derive(Default)]
pub struct MapFade {
    show: bool,
    f_image: usize,
    f_tmr: f32,
    f_alpha: isize,
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
        let mut rect = Rect::new(renderer, 0);
        rect.set_size(Vec2::new(screen_size.width, screen_size.height))
            .set_position(Vec3::new(0.0, 0.0, ORDER_MAP_FADE))
            .set_color(Color::rgba(0, 0, 0, 0));
        self.f_image = gfx_collection.add_rect(rect, 4);

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
                self.f_image,
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
            .set_color(self.f_image, Color::rgba(0, 0, 0, self.f_alpha as u8));
        self.f_type = fade_type;
        self.f_end_index = fade_end_index;
        self.f_data = fade_data;
        self.show = true;
    }
}

pub fn fade_end(
    systems: &mut SystemHolder,
    world: &mut World,
    content: &mut Content,
    socket: &mut Socket,
) -> Result<()> {
    #[allow(clippy::single_match)]
    match systems.fade.f_end_index {
        FADE_SWITCH_TO_GAME => {
            content.switch_content(world, systems, ContentType::Game)?;

            let pos = if let Some(entity) = content.game_content.myentity {
                if content.game_content.in_game {
                    world.get_or_err::<Position>(&entity)?
                } else {
                    Position::default()
                }
            } else {
                Position::default()
            };
            content.game_content.init_map(systems, pos.map)?;
            content
                .game_content
                .init_finalized_data(world, systems, socket)?;

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
