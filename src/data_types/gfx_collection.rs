use crate::info;
use cosmic_text::Attrs;
use graphics::*;
use slab::Slab;
use slotmap::SlotMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GfxType {
    Image,
    Rect,
    Text,
    Map,
}

pub struct Gfx {
    pub key: Index,
    pub gfx_type: GfxType,
    pub layer: usize,
    pub visible: bool,
    pub identifier: String,
}

#[derive(Default)]
pub struct GfxCollection {
    pub collection: Slab<Gfx>,
    pub image_storage: SlotMap<Index, Image>,
    pub rect_storage: SlotMap<Index, Rect>,
    pub text_storage: SlotMap<Index, Text>,
    pub map_storage: SlotMap<Index, Map>,
    pub sorted_array: Vec<(GfxType, usize)>,
    pub last_sorted_seconds: f32,
}

impl GfxCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count_collection(&self) {
        info!("Collection Size: {:?}", self.collection.len());
        info!("Image Size: {:?}", self.image_storage.len());
        info!("Rect Size: {:?}", self.rect_storage.len());
        info!("Text Size: {:?}", self.text_storage.len());
        info!("Map Size: {:?}", self.map_storage.len());
    }

    pub fn print_details(&self) {
        /*for data in self.collection.iter() {
            info!("Data {:?} Type: {:?}", data.0, data.1.identifier);
        }*/
        for data in self.sorted_array.iter() {
            info!("Data Type: {:?} Index: {:?}", data.0, data.1);
        }
    }

    pub fn sort_gfx_array(&mut self) {
        self.sorted_array.sort_by(|a, b| a.0.cmp(&b.0));
    }

    pub fn add_image(
        &mut self,
        image: Image,
        layer: usize,
        identifier: String,
    ) -> usize {
        let key = self.image_storage.insert(image);

        let gfx = Gfx {
            key,
            gfx_type: GfxType::Image,
            layer,
            visible: true,
            identifier,
        };
        let gfx_index = self.collection.insert(gfx);
        self.sorted_array.push((GfxType::Image, gfx_index));
        gfx_index
    }

    pub fn add_rect(
        &mut self,
        rect: Rect,
        layer: usize,
        identifier: String,
    ) -> usize {
        let key = self.rect_storage.insert(rect);

        let gfx = Gfx {
            key,
            gfx_type: GfxType::Rect,
            layer,
            visible: true,
            identifier,
        };
        let gfx_index = self.collection.insert(gfx);
        self.sorted_array.push((GfxType::Rect, gfx_index));
        gfx_index
    }

    pub fn add_text(
        &mut self,
        text: Text,
        layer: usize,
        identifier: String,
    ) -> usize {
        let key = self.text_storage.insert(text);

        let gfx = Gfx {
            key,
            gfx_type: GfxType::Text,
            layer,
            visible: true,
            identifier,
        };
        let gfx_index = self.collection.insert(gfx);
        self.sorted_array.push((GfxType::Text, gfx_index));
        gfx_index
    }

    pub fn add_map(
        &mut self,
        map: Map,
        layer: usize,
        identifier: String,
    ) -> usize {
        let key = self.map_storage.insert(map);

        let gfx = Gfx {
            key,
            gfx_type: GfxType::Map,
            layer,
            visible: true,
            identifier,
        };
        let gfx_index = self.collection.insert(gfx);
        self.sorted_array.push((GfxType::Map, gfx_index));
        gfx_index
    }

    pub fn remove_gfx(&mut self, renderer: &mut GpuRenderer, index: usize) {
        if let Some(data) = self.collection.get(index) {
            match &data.gfx_type {
                GfxType::Image => {
                    if let Some(gfx) = self.image_storage.get_mut(data.key) {
                        gfx.unload(renderer);
                    }
                    self.image_storage.remove(data.key);
                }
                GfxType::Rect => {
                    if let Some(gfx) = self.rect_storage.get_mut(data.key) {
                        gfx.unload(renderer);
                    }
                    self.rect_storage.remove(data.key);
                }
                GfxType::Text => {
                    if let Some(gfx) = self.text_storage.get_mut(data.key) {
                        gfx.unload(renderer);
                    }
                    self.text_storage.remove(data.key);
                }
                GfxType::Map => {
                    if let Some(gfx) = self.map_storage.get_mut(data.key) {
                        gfx.unload(renderer);
                    }
                    self.map_storage.remove(data.key);
                }
            }
        }
        if self.collection.contains(index) {
            self.collection.remove(index);
        }
        let index = self.sorted_array.iter().position(|data| data.1 == index);
        if let Some(gfx_index) = index {
            self.sorted_array.swap_remove(gfx_index);
        }
    }

    pub fn set_visible(&mut self, index: usize, visible: bool) {
        self.collection[index].visible = visible;
        if self.collection[index].visible {
            match &self.collection[index].gfx_type {
                GfxType::Image => {
                    if let Some(gfx) =
                        self.image_storage.get_mut(self.collection[index].key)
                    {
                        gfx.changed = true;
                    }
                }
                GfxType::Rect => {
                    if let Some(gfx) =
                        self.rect_storage.get_mut(self.collection[index].key)
                    {
                        gfx.changed = true;
                    }
                }
                GfxType::Text => {
                    if let Some(gfx) =
                        self.text_storage.get_mut(self.collection[index].key)
                    {
                        gfx.changed = true;
                    }
                }
                GfxType::Map => {
                    if let Some(gfx) =
                        self.map_storage.get_mut(self.collection[index].key)
                    {
                        gfx.changed = true;
                    }
                }
            }
        }
    }

    pub fn set_image(&mut self, index: usize, texture: usize) {
        if let Some(data) = self.collection.get(index) {
            if data.gfx_type == GfxType::Image {
                if let Some(gfx) = self.image_storage.get_mut(data.key) {
                    gfx.texture = Some(texture);
                    gfx.changed = true;
                }
            }
        }
    }

    pub fn set_color(&mut self, index: usize, color: Color) {
        if let Some(data) = self.collection.get(index) {
            match data.gfx_type {
                GfxType::Image => {
                    if let Some(gfx) = self.image_storage.get_mut(data.key) {
                        gfx.color = color;
                        gfx.changed = true;
                    }
                }
                GfxType::Rect => {
                    if let Some(gfx) = self.rect_storage.get_mut(data.key) {
                        gfx.set_color(color);
                    }
                }
                GfxType::Text => {
                    if let Some(gfx) = self.text_storage.get_mut(data.key) {
                        gfx.set_default_color(color);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn set_border_color(&mut self, index: usize, color: Color) {
        if let Some(data) = self.collection.get(index) {
            if data.gfx_type == GfxType::Rect {
                if let Some(gfx) = self.rect_storage.get_mut(data.key) {
                    gfx.set_border_color(color);
                }
            }
        }
    }

    pub fn set_border_width(&mut self, index: usize, width: f32) {
        if let Some(data) = self.collection.get(index) {
            if data.gfx_type == GfxType::Rect {
                if let Some(gfx) = self.rect_storage.get_mut(data.key) {
                    gfx.set_border_width(width);
                }
            }
        }
    }

    pub fn set_pos(&mut self, index: usize, pos: Vec3) {
        if let Some(data) = self.collection.get(index) {
            match data.gfx_type {
                GfxType::Image => {
                    if let Some(gfx) = self.image_storage.get_mut(data.key) {
                        gfx.pos = pos;
                        gfx.changed = true;
                    }
                }
                GfxType::Rect => {
                    if let Some(gfx) = self.rect_storage.get_mut(data.key) {
                        if gfx.position == pos {
                            return;
                        }
                        gfx.set_position(pos);
                    }
                }
                GfxType::Text => {
                    if let Some(gfx) = self.text_storage.get_mut(data.key) {
                        if gfx.pos == pos {
                            return;
                        }
                        gfx.set_position(pos);
                    }
                }
                GfxType::Map => {
                    if let Some(gfx) = self.map_storage.get_mut(data.key) {
                        if gfx.pos == Vec2::new(pos.x, pos.y) {
                            return;
                        }
                        gfx.pos = Vec2::new(pos.x, pos.y);
                        gfx.changed = true;
                    }
                }
            }
        }
    }

    pub fn set_bound(&mut self, index: usize, bound: Bounds) {
        if let Some(data) = self.collection.get(index) {
            if data.gfx_type == GfxType::Text {
                if let Some(gfx) = self.text_storage.get_mut(data.key) {
                    gfx.set_bounds(Some(bound));
                }
            }
        }
    }

    pub fn set_size(&mut self, index: usize, size: Vec2) {
        if let Some(data) = self.collection.get(index) {
            match data.gfx_type {
                GfxType::Image => {
                    if let Some(gfx) = self.image_storage.get_mut(data.key) {
                        gfx.hw = size;
                        gfx.changed = true;
                    }
                }
                GfxType::Rect => {
                    if let Some(gfx) = self.rect_storage.get_mut(data.key) {
                        gfx.set_size(size);
                    }
                }
                GfxType::Text => {
                    if let Some(gfx) = self.text_storage.get_mut(data.key) {
                        gfx.size = size;
                        gfx.changed = true;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn set_uv(&mut self, index: usize, uv: Vec4) {
        if let Some(data) = self.collection.get(index) {
            if data.gfx_type == GfxType::Image {
                if let Some(gfx) = self.image_storage.get_mut(data.key) {
                    gfx.uv = uv;
                    gfx.changed = true;
                }
            }
        }
    }

    pub fn set_text(
        &mut self,
        renderer: &mut GpuRenderer,
        index: usize,
        msg: &str,
    ) {
        if let Some(data) = self.collection.get(index) {
            if data.gfx_type == GfxType::Text {
                if let Some(gfx) = self.text_storage.get_mut(data.key) {
                    gfx.set_text(
                        renderer,
                        msg,
                        Attrs::new(),
                        Shaping::Advanced,
                    );
                }
            }
        }
    }

    pub fn set_rich_text<'s, 'r, I>(
        &mut self,
        renderer: &mut GpuRenderer,
        index: usize,
        msg: I,
    ) where
        I: IntoIterator<Item = (&'s str, Attrs<'r>)>,
    {
        if let Some(data) = self.collection.get(index) {
            if data.gfx_type == GfxType::Text {
                if let Some(gfx) = self.text_storage.get_mut(data.key) {
                    gfx.set_rich_text(
                        renderer,
                        msg,
                        Attrs::new(),
                        Shaping::Advanced,
                    );
                }
            }
        }
    }

    pub fn set_text_wrap(
        &mut self,
        renderer: &mut GpuRenderer,
        index: usize,
        can_wrap: bool,
    ) {
        if let Some(data) = self.collection.get(index) {
            if data.gfx_type == GfxType::Text {
                if let Some(gfx) = self.text_storage.get_mut(data.key) {
                    if can_wrap {
                        gfx.set_wrap(renderer, cosmic_text::Wrap::Word);
                    } else {
                        gfx.set_wrap(renderer, cosmic_text::Wrap::None);
                    }
                }
            }
        }
    }

    pub fn center_text(&mut self, index: usize) {
        if let Some(data) = self.collection.get(index) {
            if data.gfx_type == GfxType::Text {
                if let Some(gfx) = self.text_storage.get_mut(data.key) {
                    let size = gfx.measure();
                    let bound = gfx.bounds.unwrap_or_default();
                    let textbox_size = bound.right - bound.left;
                    gfx.pos.x =
                        bound.left + ((textbox_size * 0.5) - (size.x * 0.5));
                    gfx.changed = true;
                }
            }
        }
    }

    pub fn get_pos(&mut self, index: usize) -> Vec3 {
        if let Some(data) = self.collection.get(index) {
            match data.gfx_type {
                GfxType::Image => {
                    if let Some(gfx) = self.image_storage.get(data.key) {
                        return gfx.pos;
                    }
                }
                GfxType::Rect => {
                    if let Some(gfx) = self.rect_storage.get(data.key) {
                        return gfx.position;
                    }
                }
                GfxType::Text => {
                    if let Some(gfx) = self.text_storage.get(data.key) {
                        return gfx.pos;
                    }
                }
                GfxType::Map => {
                    if let Some(gfx) = self.map_storage.get(data.key) {
                        return Vec3::new(gfx.pos.x, gfx.pos.y, 0.0);
                    }
                }
            }
        }
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn get_size(&mut self, index: usize) -> Vec2 {
        if let Some(data) = self.collection.get(index) {
            match data.gfx_type {
                GfxType::Image => {
                    if let Some(gfx) = self.image_storage.get(data.key) {
                        return gfx.hw;
                    }
                }
                GfxType::Rect => {
                    if let Some(gfx) = self.rect_storage.get(data.key) {
                        return gfx.size;
                    }
                }
                GfxType::Text => {
                    if let Some(gfx) = self.text_storage.get(data.key) {
                        return gfx.size;
                    }
                }
                _ => return Vec2::new(0.0, 0.0),
            }
        }
        Vec2::new(0.0, 0.0)
    }

    pub fn get_uv(&mut self, index: usize) -> Vec4 {
        if let Some(data) = self.collection.get(index) {
            if data.gfx_type == GfxType::Image {
                if let Some(gfx) = self.image_storage.get(data.key) {
                    return gfx.uv;
                }
            }
        }
        Vec4::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn get_color(&mut self, index: usize) -> Color {
        if let Some(data) = self.collection.get(index) {
            match data.gfx_type {
                GfxType::Image => {
                    if let Some(gfx) = self.image_storage.get(data.key) {
                        return gfx.color;
                    }
                }
                GfxType::Rect => {
                    if let Some(gfx) = self.rect_storage.get(data.key) {
                        return gfx.color;
                    }
                }
                GfxType::Text => {
                    if let Some(gfx) = self.text_storage.get(data.key) {
                        return gfx.default_color;
                    }
                }
                _ => return Color::rgba(0, 0, 0, 0),
            }
        }
        Color::rgba(0, 0, 0, 0)
    }

    pub fn get_measure(&mut self, index: usize) -> Vec2 {
        if let Some(data) = self.collection.get(index) {
            match data.gfx_type {
                GfxType::Text => {
                    if let Some(gfx) = self.text_storage.get(data.key) {
                        return gfx.measure();
                    }
                }
                _ => return Vec2::new(0.0, 0.0),
            }
        }
        Vec2::new(0.0, 0.0)
    }

    pub fn set_map_tile(
        &mut self,
        index: usize,
        pos: (u32, u32, u32),
        tile: TileData,
    ) {
        if let Some(data) = self.collection.get(index) {
            if data.gfx_type == GfxType::Map {
                if let Some(gfx) = self.map_storage.get_mut(data.key) {
                    gfx.set_tile(pos, tile);
                }
            }
        }
    }
}
