use graphics::*;

use crate::{
    content::*, data_types::*, widget::*, Config, SystemHolder, SCREEN_WIDTH,
};

pub fn create_login(systems: &mut SystemHolder, content: &mut MenuContent) {
    let size =
        Vec2::new(348.0 * systems.scale as f32, 226.0 * systems.scale as f32)
            .floor();
    let pos = Vec2::new((SCREEN_WIDTH as f32 - size.x) * 0.5, 80.0).floor();

    let mut menu_rect = Rect::new(&mut systems.renderer, 0);
    menu_rect
        .set_position(Vec3::new(pos.x - 1.0, pos.y - 1.0, ORDER_MENU_WINDOW))
        .set_size(size + 2.0)
        .set_color(Color::rgba(160, 160, 160, 255))
        .set_border_color(Color::rgba(10, 10, 10, 255))
        .set_border_width(1.0);
    content.window.push(systems.gfx.add_rect(
        menu_rect,
        0,
        "Login Window".into(),
    ));

    let mut header_rect = Rect::new(&mut systems.renderer, 0);
    header_rect
        .set_position(Vec3::new(
            pos.x,
            pos.y + (196.0 * systems.scale as f32).floor(),
            ORDER_MENU_WINDOW_CONTENT,
        ))
        .set_size(Vec2::new(size.x, (30.0 * systems.scale as f32).floor()))
        .set_color(Color::rgba(120, 120, 120, 255));
    content.window.push(systems.gfx.add_rect(
        header_rect,
        0,
        "Login Header".into(),
    ));

    let header_text = create_label(
        systems,
        Vec3::new(
            pos.x,
            pos.y + (199.0 * systems.scale as f32).floor(),
            ORDER_MENU_WINDOW_CONTENT_DETAIL,
        ),
        Vec2::new(size.x, (20.0 * systems.scale as f32).floor()),
        Bounds::new(
            pos.x,
            pos.y + (199.0 * systems.scale as f32).floor(),
            pos.x + size.x,
            pos.y + (219.0 * systems.scale as f32).floor(),
        ),
        Color::rgba(240, 240, 240, 255),
    );
    let text_index =
        systems
            .gfx
            .add_text(header_text, 1, "Login Header Text".into());
    systems
        .gfx
        .set_text(&mut systems.renderer, text_index, "Login Window");
    systems.gfx.center_text(text_index);
    content.label.push(text_index);

    for index in 0..2 {
        let mut labelbox = Rect::new(&mut systems.renderer, 0);
        let mut textbox = Rect::new(&mut systems.renderer, 0);
        let addy = match index {
            1 => 123.0,
            _ => 154.0,
        };
        labelbox
            .set_position(Vec3::new(
                pos.x + (24.0 * systems.scale as f32).floor(),
                pos.y + (addy * systems.scale as f32).floor(),
                ORDER_MENU_WINDOW_CONTENT,
            ))
            .set_size((Vec2::new(116.0, 24.0) * systems.scale as f32).floor())
            .set_color(Color::rgba(208, 208, 208, 255));
        textbox
            .set_position(Vec3::new(
                pos.x + (140.0 * systems.scale as f32).floor(),
                pos.y + (addy * systems.scale as f32).floor(),
                ORDER_MENU_WINDOW_CONTENT,
            ))
            .set_size((Vec2::new(184.0, 24.0) * systems.scale as f32).floor())
            .set_color(Color::rgba(90, 90, 90, 255));
        content.window.push(systems.gfx.add_rect(
            labelbox,
            0,
            "Login Labelbox".into(),
        ));
        content.window.push(systems.gfx.add_rect(
            textbox,
            0,
            "Login Textbox BG".into(),
        ));

        let tpos = Vec2::new(
            pos.x + (27.0 * systems.scale as f32).floor(),
            pos.y + ((addy + 2.0) * systems.scale as f32).floor(),
        );
        let text = create_label(
            systems,
            Vec3::new(tpos.x, tpos.y, ORDER_MENU_WINDOW_CONTENT_DETAIL),
            (Vec2::new(110.0, 20.0) * systems.scale as f32).floor(),
            Bounds::new(
                tpos.x,
                tpos.y,
                tpos.x + (110.0 * systems.scale as f32).floor(),
                tpos.y + (20.0 * systems.scale as f32).floor(),
            ),
            Color::rgba(100, 100, 100, 255),
        );
        let textindex = systems.gfx.add_text(text, 1, "Login Label".into());
        let (msg, disable_option) = match index {
            1 => (
                "Password",
                vec![
                    TextDisable::Selection,
                    TextDisable::Copy,
                    TextDisable::Paste,
                ],
            ),
            _ => ("Email", vec![]),
        };
        systems.gfx.set_text(&mut systems.renderer, textindex, msg);
        content.label.push(textindex);

        let is_hidden = index == 1;

        let mut textbox = Textbox::new(
            systems,
            Vec3::new(pos.x, pos.y, ORDER_MENU_WINDOW_CONTENT_DETAIL),
            Vec2::new(142.0, addy + 2.0),
            (0.01, 2),
            Vec2::new(180.0, 20.0),
            Color::rgba(240, 240, 240, 255),
            1,
            255,
            Color::rgba(120, 120, 120, 255),
            Color::rgba(10, 10, 150, 255),
            is_hidden,
            true,
            None,
            disable_option,
        );

        match index {
            1 => textbox.set_text(systems, systems.config.password.clone()),
            _ => textbox.set_text(systems, systems.config.username.clone()),
        }
        content.textbox.push(textbox);
    }

    let button = Button::new(
        systems,
        ButtonType::Rect(ButtonRect {
            rect_color: Color::rgba(100, 100, 100, 255),
            got_border: true,
            border_color: Color::rgba(70, 70, 70, 255),
            border_radius: 0.0,
            hover_change: ButtonChangeType::ColorChange(Color::rgba(
                180, 180, 180, 255,
            )),
            click_change: ButtonChangeType::ColorChange(Color::rgba(
                40, 40, 40, 255,
            )),
        }),
        ButtonContentType::Text(ButtonContentText {
            text: "Login".to_string(),
            pos: Vec2::new(0.0, 7.0),
            color: Color::rgba(230, 230, 230, 255),
            render_layer: 1,
            hover_change: ButtonChangeType::ColorChange(Color::rgba(
                80, 80, 80, 255,
            )),
            click_change: ButtonChangeType::ColorChange(Color::rgba(
                170, 170, 170, 255,
            )),
        }),
        Vec2::new(pos.x, pos.y),
        Vec2::new(104.0, 45.0),
        ORDER_MENU_WINDOW_CONTENT,
        (0.01, 2),
        Vec2::new(140.0, 34.0),
        0,
        true,
        None,
    );
    content.button.push(button);

    let button = Button::new(
        systems,
        ButtonType::None,
        ButtonContentType::Text(ButtonContentText {
            text: "Register".to_string(),
            pos: Vec2::new(0.0, 0.0),
            color: Color::rgba(80, 80, 80, 255),
            render_layer: 1,
            hover_change: ButtonChangeType::ColorChange(Color::rgba(
                240, 240, 240, 255,
            )),
            click_change: ButtonChangeType::ColorChange(Color::rgba(
                80, 80, 80, 255,
            )),
        }),
        Vec2::new(pos.x, pos.y),
        Vec2::new(104.0, 19.0),
        ORDER_MENU_WINDOW_CONTENT,
        (0.01, 2),
        Vec2::new(140.0, 20.0),
        0,
        true,
        None,
    );
    content.button.push(button);

    let mut checkbox = Checkbox::new(
        systems,
        CheckboxType::Rect(CheckboxRect {
            rect_color: Color::rgba(100, 100, 100, 255),
            got_border: true,
            border_color: Color::rgba(50, 50, 50, 255),
            border_radius: 2.0,
            hover_change: CheckboxChangeType::ColorChange(Color::rgba(
                140, 140, 140, 255,
            )),
            click_change: CheckboxChangeType::ColorChange(Color::rgba(
                70, 70, 70, 255,
            )),
        }),
        CheckType::SetRect(CheckRect {
            rect_color: Color::rgba(200, 200, 200, 255),
            got_border: false,
            border_color: Color::rgba(255, 255, 255, 255),
            border_radius: 2.0,
            pos: Vec2::new(5.0, 5.0),
            size: Vec2::new(14.0, 14.0),
        }),
        Vec2::new(pos.x, pos.y),
        Vec2::new(116.0, 92.0),
        ORDER_MENU_WINDOW_CONTENT,
        (0.01, 2),
        Vec2::new(24.0, 24.0),
        0,
        Some(CheckboxText {
            text: "Remember account?".to_string(),
            offset_pos: Vec2::new(3.0, 2.0),
            render_layer: 1,
            label_size: Vec2::new(180.0, 20.0),
            color: Color::rgba(80, 80, 80, 255),
            hover_change: CheckboxChangeType::ColorChange(Color::rgba(
                240, 240, 240, 255,
            )),
            click_change: CheckboxChangeType::ColorChange(Color::rgba(
                80, 80, 80, 255,
            )),
        }),
        true,
        None,
    );
    checkbox.set_value(systems, systems.config.save_password);
    content.checkbox.push(checkbox);
}
