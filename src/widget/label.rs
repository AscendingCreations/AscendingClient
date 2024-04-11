use cosmic_text::{Attrs, Metrics};
use graphics::{cosmic_text::Wrap, *};

use crate::SystemHolder;

pub fn create_label(
    systems: &mut SystemHolder,
    pos: Vec3,
    label_size: Vec2,
    bounds: Bounds,
    color: Color,
) -> Text {
    let mut text = Text::new(
        &mut systems.renderer,
        Some(Metrics::new(16.0, 16.0).scale(systems.scale as f32)),
        Vec3::new(pos.x, pos.y, pos.z),
        label_size,
        1.0,
    );
    text.set_buffer_size(
        &mut systems.renderer,
        systems.size.width as i32,
        systems.size.height as i32,
    )
    .set_bounds(Some(bounds))
    .set_default_color(color);
    text.use_camera = true;
    text.changed = true;
    text
}

pub fn create_empty_label(systems: &mut SystemHolder) -> Text {
    let mut text = Text::new(
        &mut systems.renderer,
        Some(Metrics::new(16.0, 16.0).scale(systems.scale as f32)),
        Vec3::new(0.0, 0.0, 0.0),
        Vec2::new(0.0, 0.0),
        1.0,
    );
    text.set_buffer_size(
        &mut systems.renderer,
        systems.size.width as i32,
        systems.size.height as i32,
    )
    .set_bounds(Some(Bounds::new(0.0, 0.0, 0.0, 0.0)))
    .set_default_color(Color::rgba(255, 255, 255, 255));
    text.use_camera = true;
    text.changed = true;
    text
}

pub fn measure_string(systems: &mut SystemHolder, text: String) -> Vec2 {
    Text::measure_string(
        &mut systems.renderer.font_sys,
        &text,
        Attrs::new(),
        TextOptions {
            shaping: Shaping::Advanced,
            metrics: Some(Metrics::new(16.0, 16.0).scale(systems.scale as f32)),
            buffer_size: Vec2::new(systems.size.width, systems.size.height),
            scale: 1.0,
            wrap: Wrap::None,
        },
    )
}
