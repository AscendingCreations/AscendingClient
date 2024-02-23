use graphics::*;
use winit::dpi::PhysicalSize;

pub fn get_screen_center(size: &PhysicalSize<f32>) -> Vec2 {
    Vec2::new((size.width * 0.5).floor(),
        (size.height * 0.5).floor())
}