use crate::{get_percent, values::*, widget::*, SystemHolder};
use graphics::*;

pub struct VitalBar {
    bg: usize,
    bar_bg: [usize; 2],
    bar: [usize; 2],
    bar_size: f32,
}

impl VitalBar {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let size = Vec2::new(200.0, 55.0);
        let pos = Vec3::new(
            10.0,
            systems.size.height - (size.y + 10.0),
            ORDER_VITAL_BG,
        );

        let mut bg_rect = Rect::new(&mut systems.renderer, 0);
        bg_rect
            .set_position(Vec3::new(pos.x - 1.0, pos.y - 1.0, pos.z))
            .set_size(Vec2::new(size.x + 2.0, size.y + 2.0))
            .set_color(Color::rgba(180, 180, 180, 255))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(40, 40, 40, 255));
        let bg = systems.gfx.add_rect(bg_rect, 0);

        let bar_size = size.x - 12.0;

        let mut bar_bg = [0; 2];
        let mut bar = [0; 2];
        for i in 0..2 {
            let (add_y, color) = match i {
                0 => (25.0, Color::rgba(200, 80, 80, 255)),
                _ => (0.0, Color::rgba(100, 200, 80, 255)),
            };

            let mut bg_rect = Rect::new(&mut systems.renderer, 0);
            bg_rect
                .set_position(Vec3::new(
                    pos.x + 5.0,
                    pos.y + 5.0 + add_y,
                    ORDER_VITAL_HPBG,
                ))
                .set_size(Vec2::new(size.x - 10.0, 20.0))
                .set_color(Color::rgba(100, 100, 100, 255))
                .set_border_width(1.0)
                .set_border_color(Color::rgba(60, 60, 60, 255));
            bar_bg[i] = systems.gfx.add_rect(bg_rect, 0);

            let mut bar_rect = Rect::new(&mut systems.renderer, 0);
            bar_rect
                .set_position(Vec3::new(
                    pos.x + 6.0,
                    pos.y + 6.0 + add_y,
                    ORDER_VITAL_HP,
                ))
                .set_size(Vec2::new(size.x - 12.0, 18.0))
                .set_color(color);
            bar[i] = systems.gfx.add_rect(bar_rect, 0);
        }

        VitalBar {
            bg,
            bar_bg,
            bar,
            bar_size,
        }
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(self.bg);
        self.bar.iter().for_each(|index| {
            systems.gfx.remove_gfx(*index);
        });
        self.bar_bg.iter().for_each(|index| {
            systems.gfx.remove_gfx(*index);
        });
    }

    pub fn update_bar_size(
        &mut self,
        systems: &mut SystemHolder,
        bar_index: usize,
        vitals: i32,
        max_vitals: i32,
    ) {
        let mut size = systems.gfx.get_size(self.bar[bar_index]);
        size.x = get_percent(vitals, max_vitals, self.bar_size as i32) as f32;
        systems.gfx.set_size(self.bar[bar_index], size);
    }
}

pub fn create_menu_button(systems: &mut SystemHolder) -> [Button; 3] {
    let button_properties = ButtonRect {
        rect_color: Color::rgba(80, 80, 80, 255),
        got_border: true,
        border_color: Color::rgba(40, 40, 40, 255),
        border_radius: 8.0,
        hover_change: ButtonChangeType::ColorChange(Color::rgba(
            135, 135, 135, 255,
        )),
        click_change: ButtonChangeType::ColorChange(Color::rgba(
            200, 200, 200, 255,
        )),
    };
    let mut image_properties = ButtonContentImg {
        res: systems.resource.button_icon.allocation,
        pos: Vec2::new(4.0, 4.0),
        uv: Vec2::new(0.0, 0.0),
        size: Vec2::new(32.0, 32.0),
        hover_change: ButtonChangeType::None,
        click_change: ButtonChangeType::None,
    };

    let character_button = Button::new(
        systems,
        ButtonType::Rect(button_properties.clone()),
        ButtonContentType::Image(image_properties.clone()),
        Vec2::new(systems.size.width - 140.0, 10.0),
        Vec2::new(0.0, 0.0),
        ORDER_GUI_BUTTON,
        (0.01, 2),
        Vec2::new(40.0, 40.0),
        0,
        true,
        None,
    );
    image_properties.uv.x = 32.0;
    let inventory_button = Button::new(
        systems,
        ButtonType::Rect(button_properties.clone()),
        ButtonContentType::Image(image_properties.clone()),
        Vec2::new(systems.size.width - 95.0, 10.0),
        Vec2::new(0.0, 0.0),
        ORDER_GUI_BUTTON,
        (0.01, 2),
        Vec2::new(40.0, 40.0),
        0,
        true,
        None,
    );
    image_properties.uv.x = 64.0;
    let setting_button = Button::new(
        systems,
        ButtonType::Rect(button_properties.clone()),
        ButtonContentType::Image(image_properties.clone()),
        Vec2::new(systems.size.width - 50.0, 10.0),
        Vec2::new(0.0, 0.0),
        ORDER_GUI_BUTTON,
        (0.01, 2),
        Vec2::new(40.0, 40.0),
        0,
        true,
        None,
    );

    [character_button, inventory_button, setting_button]
}
