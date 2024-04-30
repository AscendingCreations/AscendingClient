use graphics::{
    cosmic_text::{rustybuzz::ttf_parser::name, Attrs},
    *,
};

use crate::{
    create_empty_label, create_label, data_types::ItemTypes, GfxType,
    SystemHolder, ORDER_ITEM_DESC, ORDER_ITEM_DESC_TEXT,
};

struct DescData {
    index: GfxType,
    pos: Vec2,
}

struct ItemDescData {
    index: usize,
    data: Vec<DescData>,
    size: Vec2,
}

pub struct ItemDescription {
    pub visible: bool,
    bg: GfxType,
    data: Option<ItemDescData>,
    size: Vec2,
    min_bound: Vec2,
    max_bound: Vec2,
}

impl ItemDescription {
    pub fn new(systems: &mut SystemHolder) -> Self {
        let mut bg_rect = Rect::new(&mut systems.renderer, 0);
        bg_rect
            .set_position(Vec3::new(0.0, 0.0, 0.0))
            .set_size(Vec2::new(0.0, 0.0))
            .set_color(Color::rgba(110, 110, 110, 255))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(40, 40, 40, 255));
        let bg =
            systems
                .gfx
                .add_rect(bg_rect, 0, "Item Desc Window".into(), false);

        ItemDescription {
            visible: false,
            bg,
            data: None,
            size: Vec2::new(0.0, 0.0),
            min_bound: Vec2::new(0.0, 0.0),
            max_bound: Vec2::new(0.0, 0.0),
        }
    }

    pub fn unload(&mut self, systems: &mut SystemHolder) {
        systems.gfx.remove_gfx(&mut systems.renderer, &self.bg);
        if let Some(data) = &self.data {
            for desc_data in data.data.iter() {
                systems
                    .gfx
                    .remove_gfx(&mut systems.renderer, &desc_data.index);
            }
        }
    }

    pub fn set_visible(&mut self, systems: &mut SystemHolder, visible: bool) {
        self.visible = visible;
        systems.gfx.set_visible(&self.bg, visible);
        if let Some(data) = &self.data {
            for desc_data in data.data.iter() {
                systems.gfx.set_visible(&desc_data.index, visible);
            }
        }
    }

    pub fn set_data(&mut self, systems: &mut SystemHolder, index: usize) {
        if let Some(data) = &self.data {
            if data.index == index {
                return;
            } else {
                for desc_data in data.data.iter() {
                    systems
                        .gfx
                        .remove_gfx(&mut systems.renderer, &desc_data.index);
                }
            }
        }

        let item_base = &systems.base.item[index];

        let mut text_holder = Vec::with_capacity(4);
        text_holder.push(item_base.name.clone());

        match item_base.itemtype {
            ItemTypes::Consume => {
                if item_base.data[0] > 0 {
                    text_holder.push(format!("HP + {}", item_base.data[0]))
                }
                if item_base.data[1] > 0 {
                    text_holder.push(format!("MP + {}", item_base.data[1]))
                }
                if item_base.data[2] > 0 {
                    text_holder.push(format!("SP + {}", item_base.data[2]))
                }
            }
            ItemTypes::Weapon => {
                text_holder.push(format!("Damage: {}", item_base.data[0]))
            }
            ItemTypes::Accessory
            | ItemTypes::Helmet
            | ItemTypes::Armor
            | ItemTypes::Trouser => {
                text_holder.push(format!("Defense: {}", item_base.data[0]))
            }
            _ => {}
        }

        let mut name_text = create_empty_label(systems);
        let mut text_size = 0.0;
        for text in text_holder.iter() {
            name_text.set_text(
                &mut systems.renderer,
                text,
                Attrs::new(),
                Shaping::Advanced,
            );
            let name_size = name_text.measure();
            if name_size.x > text_size {
                text_size = name_size.x.floor();
            }
        }

        let mut size = Vec2::new(
            text_size + 20.0,
            20.0 + (text_holder.len() as f32 * 20.0),
        );
        if text_holder.len() > 1 {
            size.y += 5.0;
        }
        name_text.set_text(
            &mut systems.renderer,
            &text_holder[0],
            Attrs::new(),
            Shaping::Advanced,
        );

        let tpos = Vec2::new(10.0, size.y - 30.0);
        let text_size = Vec2::new(text_size, 20.0);

        name_text.size = text_size;
        name_text
            .set_position(Vec3::new(tpos.x, tpos.y, ORDER_ITEM_DESC_TEXT))
            .set_bounds(Some(Bounds::new(
                tpos.x,
                tpos.y,
                tpos.x + text_size.x,
                tpos.y + text_size.y,
            )))
            .set_default_color(Color::rgba(250, 250, 250, 255));
        let name = systems.gfx.add_text(
            name_text,
            1,
            "Item Desc Name".into(),
            self.visible,
        );

        let mut data = Vec::with_capacity(text_holder.len());
        data.push(DescData {
            index: name,
            pos: tpos,
        });

        for (index, msg) in text_holder.iter().enumerate() {
            if index != 0 {
                let n_pos =
                    Vec2::new(10.0, size.y - (35.0 + (20.0 * index as f32)));
                let text = create_label(
                    systems,
                    Vec3::new(n_pos.x, n_pos.y, ORDER_ITEM_DESC_TEXT),
                    text_size,
                    Bounds::new(
                        n_pos.x,
                        n_pos.y,
                        n_pos.x + text_size.x,
                        n_pos.y + text_size.y,
                    ),
                    Color::rgba(200, 200, 200, 255),
                );
                let text_index = systems.gfx.add_text(
                    text,
                    1,
                    "Item Desc Text".into(),
                    self.visible,
                );
                systems
                    .gfx
                    .set_text(&mut systems.renderer, &text_index, msg);

                data.push(DescData {
                    index: text_index,
                    pos: n_pos,
                });
            }
        }

        self.data = Some(ItemDescData {
            index,
            data,
            size: text_size,
        });

        systems
            .gfx
            .set_pos(&self.bg, Vec3::new(-1.0, -1.0, ORDER_ITEM_DESC));
        systems
            .gfx
            .set_size(&self.bg, Vec2::new(size.x + 2.0, size.y + 2.0));

        self.min_bound = Vec2::new(
            systems.size.width - size.x - 1.0,
            systems.size.height - size.y - 1.0,
        );
        self.max_bound = Vec2::new(1.0, 1.0);
        self.size = size;
    }

    pub fn set_position(
        &mut self,
        systems: &mut SystemHolder,
        screen_pos: Vec2,
    ) {
        let pos = Vec2::new(screen_pos.x, screen_pos.y)
            .max(self.max_bound)
            .min(self.min_bound);

        systems.gfx.set_pos(
            &self.bg,
            Vec3::new(pos.x - 1.0, pos.y - 1.0, ORDER_ITEM_DESC),
        );

        if let Some(data) = &self.data {
            for desc_data in data.data.iter() {
                systems.gfx.set_pos(
                    &desc_data.index,
                    Vec3::new(
                        pos.x + desc_data.pos.x,
                        pos.y + desc_data.pos.y,
                        ORDER_ITEM_DESC_TEXT,
                    ),
                );
                systems.gfx.set_bound(
                    &desc_data.index,
                    Bounds::new(
                        pos.x + desc_data.pos.x,
                        pos.y + desc_data.pos.y,
                        pos.x + desc_data.pos.x + data.size.x,
                        pos.y + desc_data.pos.y + data.size.y,
                    ),
                );
            }
        }
    }
}
