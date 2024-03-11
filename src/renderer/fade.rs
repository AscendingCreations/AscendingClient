use graphics::*;

use hecs::World;
use winit::dpi::PhysicalSize;

use crate::{
    gfx_collection::*, values::*, content::*, DrawSetting,
};

pub enum FadeType {
    In,
    Out,
}

pub const FADE_LOGIN: usize = 1;

pub struct Fade {
    show: bool,
    f_image: usize,
    f_tmr: f32,
    f_alpha: isize,
    f_type: FadeType,
    f_end_index: usize,
}

impl Fade {
    pub fn new() -> Self {
        Fade {
            show: false,
            f_image: 0,
            f_tmr: 0.0,
            f_alpha: 0,
            f_type: FadeType::In,
            f_end_index: 0,
        }
    }

    pub fn init_setup(&mut self, renderer: &mut GpuRenderer, gfx_collection: &mut GfxCollection, screen_size: &PhysicalSize<f32>) {
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

    pub fn fade_logic(&mut self, gfx_collection: &mut GfxCollection, seconds: f32) -> bool {
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
            gfx_collection.set_color(self.f_image, Color::rgba(0, 0, 0, self.f_alpha as u8));
            self.f_tmr = seconds + 0.03;
        }

        did_end
    }

    pub fn init_fade(&mut self, gfx_collection: &mut GfxCollection, fade_type: FadeType, fade_end_index: usize) {
        match fade_type {
            FadeType::In => {
                self.f_alpha = 0;
            }
            FadeType::Out => {
                self.f_alpha = 255;
            }
        }
        gfx_collection.set_color(self.f_image, Color::rgba(0, 0, 0, self.f_alpha as u8));
        self.f_type = fade_type;
        self.f_end_index = fade_end_index;
        self.show = true;
    }
}

pub struct MapFade {
    show: bool,
    f_image: usize,
    f_tmr: f32,
    f_alpha: isize,
    f_type: FadeType,
    f_end_index: usize,
}

impl MapFade {
    pub fn new() -> Self {
        MapFade {
            show: false,
            f_image: 0,
            f_tmr: 0.0,
            f_alpha: 0,
            f_type: FadeType::In,
            f_end_index: 0,
        }
    }

    pub fn init_setup(&mut self, renderer: &mut GpuRenderer, gfx_collection: &mut GfxCollection, screen_size: &PhysicalSize<f32>) {
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

    pub fn fade_logic(&mut self, gfx_collection: &mut GfxCollection, seconds: f32) -> bool {
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
            gfx_collection.set_color(self.f_image, Color::rgba(0, 0, 0, self.f_alpha as u8));
            self.f_tmr = seconds + 0.03;
        }

        did_end
    }

    pub fn init_fade(&mut self, gfx_collection: &mut GfxCollection, fade_type: FadeType, fade_end_index: usize) {
        match fade_type {
            FadeType::In => {
                self.f_alpha = 0;
            }
            FadeType::Out => {
                self.f_alpha = 255;
            }
        }
        gfx_collection.set_color(self.f_image, Color::rgba(0, 0, 0, self.f_alpha as u8));
        self.f_type = fade_type;
        self.f_end_index = fade_end_index;
        self.show = true;
    }
}

pub fn fade_end(
    systems: &mut DrawSetting,
    world: &mut World,
    content: &mut Content,
) {
    match systems.fade.f_end_index {
        FADE_LOGIN => {
            content.switch_content(world, systems, ContentType::Game);
            content.init_map(systems, MapPosition::new(0, 0, 0));
            if let ContentHolder::Game(data) = &mut content.holder {
                data.init_data(world, systems);
            }
            
            systems.fade.init_fade(&mut systems.gfx, FadeType::Out, 0);
        }
        _ => {}
    }
}

pub fn map_fade_end(
    systems: &mut DrawSetting,
    _world: &mut World,
    _content: &mut Content,
) {
    match systems.map_fade.f_end_index {
        1 => {            
            systems.map_fade.init_fade(&mut systems.gfx, FadeType::Out, 0);
        }
        _ => {}
    }
}