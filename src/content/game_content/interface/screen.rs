use crate::{values::*, widget::*, DrawSetting};
use graphics::*;

pub fn create_menu_button(systems: &mut DrawSetting) -> [Button; 3] {
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
