use graphics::*;

use winit::dpi::PhysicalSize;

use crate::{
    DrawSetting,
    gfx_collection::*,
    gfx_order::*,
};

pub struct Fade {
    fade_image: usize,
    show: bool,
    fade_tmr: f32,
    fade_alpha: usize,
}

impl Fade {
    pub fn new() -> Self {
        Fade {
            fade_image: 0,
            show: false,
            fade_tmr: 0.0,
            fade_alpha: 0,
        }
    }

    pub fn init_setup(&mut self, renderer: &mut GpuRenderer, gfx_collection: &mut GfxCollection, screen_size: &PhysicalSize<f32>) {
        let mut rect = Rect::new(renderer, 0);
        rect.set_size(Vec2::new(screen_size.width, screen_size.height))
            .set_position(Vec3::new(0.0, 0.0, ORDER_FADE))
            .set_color(Color::rgba(0, 0, 0, 100));
        self.fade_image = gfx_collection.add_rect(rect, 2);

        self.show = false;
        self.fade_tmr = 0.0;
        self.fade_alpha = 0;
    }

    pub fn fade_logic(&mut self, seconds: f32) {
        if !self.show {
            return;
        }
        
        if self.fade_tmr <= seconds {
            self.fade_tmr = seconds + 0.05;
        }
    } 
}