use graphics::*;

use crate::{content::*, data_types::*, widget::*, SystemHolder, SCREEN_WIDTH};

pub fn create_register(systems: &mut SystemHolder, content: &mut MenuContent) {
    let size = (Vec2::new(348.0, 375.0) * systems.scale as f32).floor();
    let pos = Vec2::new((SCREEN_WIDTH as f32 - size.x) * 0.5, 20.0).floor();

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
        "Register Window".into(),
    ));

    let mut header_rect = Rect::new(&mut systems.renderer, 0);
    header_rect
        .set_position(Vec3::new(
            pos.x,
            pos.y + (345.0 * systems.scale as f32).floor(),
            ORDER_MENU_WINDOW_CONTENT,
        ))
        .set_size(Vec2::new(size.x, (30.0 * systems.scale as f32).floor()))
        .set_color(Color::rgba(120, 120, 120, 255));
    content.window.push(systems.gfx.add_rect(
        header_rect,
        0,
        "Register Header".into(),
    ));

    let header_text = create_label(
        systems,
        Vec3::new(
            pos.x,
            pos.y + (348.0 * systems.scale as f32).floor(),
            ORDER_MENU_WINDOW_CONTENT_DETAIL,
        ),
        Vec2::new(size.x, (20.0 * systems.scale as f32).floor()),
        Bounds::new(
            pos.x,
            pos.y + (348.0 * systems.scale as f32).floor(),
            pos.x + size.x,
            pos.y + (368.0 * systems.scale as f32).floor(),
        ),
        Color::rgba(240, 240, 240, 255),
    );
    let text_index =
        systems
            .gfx
            .add_text(header_text, 1, "Register Header Text".into());
    systems
        .gfx
        .set_text(&mut systems.renderer, text_index, "Register Window");
    systems.gfx.center_text(text_index);
    content.label.push(text_index);

    for index in 0..5 {
        let mut labelbox = Rect::new(&mut systems.renderer, 0);
        let mut textbox_bg = Rect::new(&mut systems.renderer, 0);
        let addy = match index {
            1 => 278.0,
            2 => 247.0,
            3 => 222.0,
            4 => 191.0,
            _ => 303.0,
        };
        labelbox
            .set_position(Vec3::new(
                pos.x + (24.0 * systems.scale as f32).floor(),
                pos.y + (addy * systems.scale as f32).floor(),
                ORDER_MENU_WINDOW_CONTENT,
            ))
            .set_size((Vec2::new(116.0, 24.0) * systems.scale as f32).floor())
            .set_color(Color::rgba(208, 208, 208, 255));
        textbox_bg
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
            "Register Labelbox".into(),
        ));
        content.window.push(systems.gfx.add_rect(
            textbox_bg,
            0,
            "Register Textbox BG".into(),
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
        let textindex = systems.gfx.add_text(text, 1, "Register Label".into());
        let msg = match index {
            1 => "Retype",
            2 => "Password",
            3 => "Retype",
            4 => "Username",
            _ => "Email",
        };
        systems.gfx.set_text(&mut systems.renderer, textindex, msg);
        content.label.push(textindex);

        let tooltip = match index {
            0 | 1 => {
                Some("This email will be used for password reset".to_string())
            }
            _ => None,
        };

        let textbox = Textbox::new(
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
            false,
            true,
            tooltip,
            vec![],
        );

        content.textbox.push(textbox);
    }

    let mut sprite_bg = Rect::new(&mut systems.renderer, 0);
    sprite_bg
        .set_position(Vec3::new(
            pos.x + (34.0 * systems.scale as f32).floor(),
            pos.y + (98.0 * systems.scale as f32).floor(),
            ORDER_MENU_WINDOW_CONTENT,
        ))
        .set_size((Vec2::new(80.0, 80.0) * systems.scale as f32).floor())
        .set_color(Color::rgba(120, 120, 120, 255));
    content.window.push(systems.gfx.add_rect(
        sprite_bg,
        0,
        "Register Sprite BG".into(),
    ));

    let mut image = Image::new(
        Some(systems.resource.players[0].allocation),
        &mut systems.renderer,
        0,
    );
    image.hw = (Vec2::new(80.0, 80.0) * systems.scale as f32).floor();
    image.pos = Vec3::new(
        pos.x + (34.0 * systems.scale as f32).floor(),
        pos.y + (98.0 * systems.scale as f32).floor(),
        ORDER_MENU_WINDOW_CONTENT_DETAIL,
    );
    image.uv = Vec4::new(0.0, 0.0, 40.0, 40.0);
    content.image.push(systems.gfx.add_image(
        image,
        0,
        "Register Sprite".into(),
    ));

    let sprite_label = create_label(
        systems,
        Vec3::new(
            pos.x + (142.0 * systems.scale as f32).floor(),
            pos.y + (148.0 * systems.scale as f32).floor(),
            ORDER_MENU_WINDOW_CONTENT_DETAIL,
        ),
        Vec2::new(size.x, (20.0 * systems.scale as f32).floor()),
        Bounds::new(
            pos.x + (142.0 * systems.scale as f32).floor(),
            pos.y + (148.0 * systems.scale as f32).floor(),
            pos.x + (306.0 * systems.scale as f32).floor(),
            pos.y + (168.0 * systems.scale as f32).floor(),
        ),
        Color::rgba(80, 80, 80, 255),
    );
    let sprite_index =
        systems
            .gfx
            .add_text(sprite_label, 1, "Register Sprite Label".into());
    systems.gfx.set_text(
        &mut systems.renderer,
        sprite_index,
        "Sprite Selection",
    );
    systems.gfx.center_text(sprite_index);
    content.label.push(sprite_index);

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
            text: "Register".to_string(),
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
            text: "Sign In".to_string(),
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
        ButtonContentType::Image(ButtonContentImg {
            res: systems.resource.horizontal_arrow.allocation,
            pos: Vec2::new(0.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
            size: Vec2::new(24.0, 24.0),
            hover_change: ButtonChangeType::None,
            click_change: ButtonChangeType::None,
        }),
        Vec2::new(pos.x, pos.y),
        Vec2::new(142.0, 118.0),
        ORDER_MENU_WINDOW_CONTENT,
        (0.01, 2),
        Vec2::new(24.0, 24.0),
        0,
        true,
        None,
    );
    content.button.push(button);

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
        ButtonContentType::Image(ButtonContentImg {
            res: systems.resource.horizontal_arrow.allocation,
            pos: Vec2::new(0.0, 0.0),
            uv: Vec2::new(24.0, 0.0),
            size: Vec2::new(24.0, 24.0),
            hover_change: ButtonChangeType::None,
            click_change: ButtonChangeType::None,
        }),
        Vec2::new(pos.x, pos.y),
        Vec2::new(282.0, 118.0),
        ORDER_MENU_WINDOW_CONTENT,
        (0.01, 2),
        Vec2::new(24.0, 24.0),
        0,
        true,
        None,
    );
    content.button.push(button);

    let sprite_number_text = create_label(
        systems,
        Vec3::new(
            pos.x + (170.0 * systems.scale as f32).floor(),
            pos.y + (120.0 * systems.scale as f32).floor(),
            ORDER_MENU_WINDOW_CONTENT_DETAIL,
        ),
        Vec2::new(size.x, (20.0 * systems.scale as f32).floor()),
        Bounds::new(
            pos.x + (170.0 * systems.scale as f32).floor(),
            pos.y + (120.0 * systems.scale as f32).floor(),
            pos.x + (278.0 * systems.scale as f32).floor(),
            pos.y + (140.0 * systems.scale as f32).floor(),
        ),
        Color::rgba(80, 80, 80, 255),
    );
    let sprite_number = systems.gfx.add_text(
        sprite_number_text,
        1,
        "Register Sprite Number".into(),
    );
    systems
        .gfx
        .set_text(&mut systems.renderer, sprite_number, "0");
    systems.gfx.center_text(sprite_number);
    content.unique_label.push(sprite_number);
}
