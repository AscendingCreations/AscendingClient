use crate::{
    get_start_map_pos, label::*, GameContent, MapPosition, Position,
    SystemHolder, ORDER_FLOAT_TEXT, ORDER_FLOAT_TEXT_BG, TILE_SIZE,
};
use graphics::{
    cosmic_text::{Attrs, Metrics, Stretch, Style, Weight},
    *,
};
use rand::{thread_rng, Rng};

struct FloatingTextData {
    text_bg: usize,
    text: usize,
    size: Vec2,
    adjust_pos: Vec2,
    float_y: f32,
    pos: Position,
    timer: f32,
    spawned: bool,
}

pub struct FloatingText {
    unload: bool,
    data: Vec<FloatingTextData>,
}

impl FloatingText {
    pub fn new() -> Self {
        FloatingText {
            unload: false,
            data: Vec::new(),
        }
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        self.unload = true;
        for data in &self.data {
            systems.gfx.remove_gfx(data.text);
            systems.gfx.remove_gfx(data.text_bg);
        }
        self.data.clear();
    }

    pub fn recreate(&mut self) {
        self.unload = false;
    }
}

pub fn float_text_loop(
    systems: &mut SystemHolder,
    content: &mut GameContent,
    seconds: f32,
) {
    if content.float_text.unload {
        return;
    }

    let mut remove_list = Vec::new();

    for (index, float_data) in content.float_text.data.iter_mut().enumerate() {
        if float_data.spawned {
            float_data.timer = seconds;
            float_data.spawned = false;
        }

        let interval = seconds - float_data.timer;
        if interval > 1.5 {
            remove_list.push(index);
        }

        float_data.float_y += 0.2;

        let start_pos =
            get_start_map_pos(content.map.map_pos, float_data.pos.map)
                .unwrap_or_else(|| Vec2::new(0.0, 0.0));
        let cur_pos = systems.gfx.get_pos(float_data.text);
        let texture_pos = content.camera.pos
            + (Vec2::new(float_data.pos.x as f32, float_data.pos.y as f32)
                * TILE_SIZE as f32);

        let pos = Vec2::new(
            start_pos.x + texture_pos.x + float_data.adjust_pos.x,
            start_pos.y
                + texture_pos.y
                + float_data.adjust_pos.y
                + float_data.float_y,
        );

        if pos != Vec2::new(cur_pos.x, cur_pos.y) {
            systems.gfx.set_pos(
                float_data.text,
                Vec3::new(pos.x, pos.y, ORDER_FLOAT_TEXT),
            );
            systems.gfx.set_pos(
                float_data.text_bg,
                Vec3::new(pos.x - 1.0, pos.y - 2.0, ORDER_FLOAT_TEXT_BG),
            );
        }
    }

    for index in remove_list.iter().rev() {
        systems.gfx.remove_gfx(content.float_text.data[*index].text);
        systems
            .gfx
            .remove_gfx(content.float_text.data[*index].text_bg);
        content.float_text.data.swap_remove(*index);
    }
}

pub fn add_float_text(
    systems: &mut SystemHolder,
    content: &mut GameContent,
    pos: Position,
    msg: String,
    color: Color,
) {
    let start_pos = get_start_map_pos(content.map.map_pos, pos.map)
        .unwrap_or_else(|| Vec2::new(0.0, 0.0));
    let texture_pos = content.camera.pos
        + (Vec2::new(pos.x as f32, pos.y as f32) * TILE_SIZE as f32);

    let mut text = create_empty_label(systems);
    text.set_text(&mut systems.renderer, &msg, Attrs::new(), Shaping::Advanced);
    let size = Vec2::new(text.measure().x.floor(), 20.0);
    text.size = size;

    let mut rng = thread_rng();
    let add_x = rng.gen_range(-6..=6);

    let mut adjust_pos = (TILE_SIZE as f32 - size) * 0.5;
    adjust_pos.y += 8.0;
    adjust_pos.x += add_x as f32;
    let tpos = start_pos + texture_pos + adjust_pos;

    text.set_position(Vec3::new(tpos.x, tpos.y, ORDER_FLOAT_TEXT))
        .set_bounds(Some(Bounds::new(
            0.0,
            0.0,
            systems.size.width,
            systems.size.height,
        )))
        .set_default_color(color);
    let text_index = systems.gfx.add_text(text, 1);
    systems.gfx.set_visible(text_index, true);

    let mut textbg = create_label(
        systems,
        Vec3::new(tpos.x - 1.0, tpos.y - 2.0, ORDER_FLOAT_TEXT_BG),
        size,
        Bounds::new(0.0, 0.0, systems.size.width, systems.size.height),
        Color::rgba(10, 10, 10, 255),
    );
    let attrs = Attrs::new().weight(Weight::BOLD);
    textbg.set_text(&mut systems.renderer, &msg, attrs, Shaping::Advanced);
    let text_bg = systems.gfx.add_text(textbg, 1);
    systems.gfx.set_visible(text_bg, true);

    content.float_text.data.push(FloatingTextData {
        text: text_index,
        text_bg,
        size,
        pos,
        adjust_pos,
        timer: 0.0,
        spawned: true,
        float_y: 0.0,
    });
}
