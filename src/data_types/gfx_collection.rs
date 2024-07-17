use std::{borrow::Cow, default};

use crate::info;
use cosmic_text::Attrs;
use graphics::*;
use slab::Slab;
use slotmap::SlotMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum GfxType {
    #[default]
    None,
    Image(Index),
    Rect(Index),
    Text(Index),
    Map(Index),
    Light(Index),
}

pub struct GfxData {
    pub layer: usize,
    pub visible: bool,
    pub identifier: Cow<'static, str>,
}

pub struct GfxImage {
    pub data: GfxData,
    pub gfx: Image,
}

pub struct GfxRect {
    pub data: GfxData,
    pub gfx: Rect,
}

pub struct GfxText {
    pub data: GfxData,
    pub gfx: Text,
}

pub struct GfxMap {
    pub data: GfxData,
    pub gfx: Map,
}

pub struct GfxLight {
    pub data: GfxData,
    pub gfx: Lights,
}

#[derive(Default)]
pub struct GfxCollection {
    pub image_storage: SlotMap<Index, GfxImage>,
    pub rect_storage: SlotMap<Index, GfxRect>,
    pub text_storage: SlotMap<Index, GfxText>,
    pub map_storage: SlotMap<Index, GfxMap>,
    pub light_storage: SlotMap<Index, GfxLight>,
}

impl GfxCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn count_collection(&self) {
        info!("Image Size: {:?}", self.image_storage.len());
        info!("Rect Size: {:?}", self.rect_storage.len());
        info!("Text Size: {:?}", self.text_storage.len());
        info!("Map Size: {:?}", self.map_storage.len());
        info!("Light Size: {:?}", self.light_storage.len());
    }

    pub fn add_image(
        &mut self,
        gfx: Image,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
    ) -> GfxType {
        let data = GfxData {
            layer,
            visible,
            identifier: identifier.into(),
        };

        GfxType::Image(self.image_storage.insert(GfxImage { data, gfx }))
    }

    pub fn add_rect(
        &mut self,
        gfx: Rect,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
    ) -> GfxType {
        let data = GfxData {
            layer,
            visible,
            identifier: identifier.into(),
        };

        GfxType::Rect(self.rect_storage.insert(GfxRect { data, gfx }))
    }

    pub fn add_text(
        &mut self,
        gfx: Text,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
    ) -> GfxType {
        let data = GfxData {
            layer,
            visible,
            identifier: identifier.into(),
        };

        GfxType::Text(self.text_storage.insert(GfxText { data, gfx }))
    }

    pub fn add_map(
        &mut self,
        gfx: Map,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
    ) -> GfxType {
        let data = GfxData {
            layer,
            visible,
            identifier: identifier.into(),
        };

        GfxType::Map(self.map_storage.insert(GfxMap { data, gfx }))
    }

    pub fn add_light(
        &mut self,
        gfx: Lights,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
    ) -> GfxType {
        let data = GfxData {
            layer,
            visible,
            identifier: identifier.into(),
        };

        GfxType::Light(self.light_storage.insert(GfxLight { data, gfx }))
    }

    pub fn remove_gfx(&mut self, renderer: &mut GpuRenderer, index: &GfxType) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.remove(*gfx_index) {
                    gfx.gfx.unload(renderer);
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.remove(*gfx_index) {
                    gfx.gfx.unload(renderer);
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.remove(*gfx_index) {
                    gfx.gfx.unload(renderer);
                }
            }
            GfxType::Map(gfx_index) => {
                if let Some(gfx) = self.map_storage.remove(*gfx_index) {
                    gfx.gfx.unload(renderer);
                }
            }
            GfxType::Light(gfx_index) => {
                if let Some(gfx) = self.light_storage.remove(*gfx_index) {
                    gfx.gfx.unload(renderer);
                }
            }
            _ => {}
        }
    }

    pub fn set_visible(&mut self, index: &GfxType, visible: bool) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    gfx.data.visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    gfx.data.visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    gfx.data.visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Map(gfx_index) => {
                if let Some(gfx) = self.map_storage.get_mut(*gfx_index) {
                    gfx.data.visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Light(gfx_index) => {
                if let Some(gfx) = self.light_storage.get_mut(*gfx_index) {
                    gfx.data.visible = visible;
                    gfx.gfx.changed = true;
                }
            }
            _ => {}
        }
    }

    pub fn set_image(&mut self, index: &GfxType, texture: usize) {
        if let GfxType::Image(gfx_index) = index {
            if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                gfx.gfx.texture = Some(texture);
                gfx.gfx.changed = true;
            }
        }
    }

    pub fn set_color(&mut self, index: &GfxType, color: Color) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    gfx.gfx.color = color;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_color(color);
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_default_color(color);
                }
            }
            _ => {}
        }
    }

    pub fn set_border_color(&mut self, index: &GfxType, color: Color) {
        if let GfxType::Rect(gfx_index) = index {
            if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                gfx.gfx.set_border_color(color);
            }
        }
    }

    pub fn set_border_width(&mut self, index: &GfxType, width: f32) {
        if let GfxType::Rect(gfx_index) = index {
            if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                gfx.gfx.set_border_width(width);
            }
        }
    }

    pub fn set_pos(&mut self, index: &GfxType, pos: Vec3) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    gfx.gfx.pos = pos;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    if gfx.gfx.position == pos {
                        return;
                    }
                    gfx.gfx.set_position(pos);
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    if gfx.gfx.pos == pos {
                        return;
                    }
                    gfx.gfx.set_position(pos);
                }
            }
            GfxType::Map(gfx_index) => {
                if let Some(gfx) = self.map_storage.get_mut(*gfx_index) {
                    if gfx.gfx.pos == Vec2::new(pos.x, pos.y) {
                        return;
                    }
                    gfx.gfx.pos = Vec2::new(pos.x, pos.y);
                    gfx.gfx.changed = true;
                }
            }
            _ => {}
        }
    }

    pub fn set_bound(&mut self, index: &GfxType, bound: Bounds) {
        if let GfxType::Text(gfx_index) = index {
            if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                gfx.gfx.set_bounds(Some(bound));
            }
        }
    }

    pub fn set_size(&mut self, index: &GfxType, size: Vec2) {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                    gfx.gfx.hw = size;
                    gfx.gfx.changed = true;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get_mut(*gfx_index) {
                    gfx.gfx.set_size(size);
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                    gfx.gfx.size = size;
                    gfx.gfx.changed = true;
                }
            }
            _ => {}
        }
    }

    pub fn set_uv(&mut self, index: &GfxType, uv: Vec4) {
        if let GfxType::Image(gfx_index) = index {
            if let Some(gfx) = self.image_storage.get_mut(*gfx_index) {
                gfx.gfx.uv = uv;
                gfx.gfx.changed = true;
            }
        }
    }

    pub fn set_text(
        &mut self,
        renderer: &mut GpuRenderer,
        index: &GfxType,
        msg: &str,
    ) {
        if let GfxType::Text(gfx_index) = index {
            if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                gfx.gfx.set_text(
                    renderer,
                    msg,
                    Attrs::new(),
                    Shaping::Advanced,
                );
            }
        }
    }

    pub fn set_rich_text<'s, 'r, I>(
        &mut self,
        renderer: &mut GpuRenderer,
        index: &GfxType,
        msg: I,
    ) where
        I: IntoIterator<Item = (&'s str, Attrs<'r>)>,
    {
        if let GfxType::Text(gfx_index) = index {
            if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                gfx.gfx.set_rich_text(
                    renderer,
                    msg,
                    Attrs::new(),
                    Shaping::Advanced,
                );
            }
        }
    }

    pub fn set_text_wrap(
        &mut self,
        renderer: &mut GpuRenderer,
        index: &GfxType,
        can_wrap: bool,
    ) {
        if let GfxType::Text(gfx_index) = index {
            if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                if can_wrap {
                    gfx.gfx.set_wrap(renderer, cosmic_text::Wrap::Word);
                } else {
                    gfx.gfx.set_wrap(renderer, cosmic_text::Wrap::None);
                }
            }
        }
    }

    pub fn center_text(&mut self, index: &GfxType) {
        if let GfxType::Text(gfx_index) = index {
            if let Some(gfx) = self.text_storage.get_mut(*gfx_index) {
                let size = gfx.gfx.measure();
                let bound = gfx.gfx.bounds.unwrap_or_default();
                let textbox_size = bound.right - bound.left;
                gfx.gfx.pos.x =
                    bound.left + ((textbox_size * 0.5) - (size.x * 0.5));
                gfx.gfx.changed = true;
            }
        }
    }

    pub fn get_pos(&mut self, index: &GfxType) -> Vec3 {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get(*gfx_index) {
                    return gfx.gfx.pos;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get(*gfx_index) {
                    return gfx.gfx.position;
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get(*gfx_index) {
                    return gfx.gfx.pos;
                }
            }
            GfxType::Map(gfx_index) => {
                if let Some(gfx) = self.map_storage.get(*gfx_index) {
                    return Vec3::new(gfx.gfx.pos.x, gfx.gfx.pos.y, 0.0);
                }
            }
            _ => {}
        }

        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn get_size(&mut self, index: &GfxType) -> Vec2 {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get(*gfx_index) {
                    return gfx.gfx.hw;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get(*gfx_index) {
                    return gfx.gfx.size;
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get(*gfx_index) {
                    return gfx.gfx.size;
                }
            }
            _ => return Vec2::new(0.0, 0.0),
        }

        Vec2::new(0.0, 0.0)
    }

    pub fn get_uv(&mut self, index: &GfxType) -> Vec4 {
        if let GfxType::Image(gfx_index) = index {
            if let Some(gfx) = self.image_storage.get(*gfx_index) {
                return gfx.gfx.uv;
            }
        }

        Vec4::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn get_color(&mut self, index: &GfxType) -> Color {
        match index {
            GfxType::Image(gfx_index) => {
                if let Some(gfx) = self.image_storage.get(*gfx_index) {
                    return gfx.gfx.color;
                }
            }
            GfxType::Rect(gfx_index) => {
                if let Some(gfx) = self.rect_storage.get(*gfx_index) {
                    return gfx.gfx.color;
                }
            }
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get(*gfx_index) {
                    return gfx.gfx.default_color;
                }
            }
            _ => return Color::rgba(0, 0, 0, 0),
        }

        Color::rgba(0, 0, 0, 0)
    }

    pub fn get_measure(&mut self, index: &GfxType) -> Vec2 {
        match index {
            GfxType::Text(gfx_index) => {
                if let Some(gfx) = self.text_storage.get(*gfx_index) {
                    return gfx.gfx.measure();
                }
            }
            _ => return Vec2::new(0.0, 0.0),
        }

        Vec2::new(0.0, 0.0)
    }

    pub fn set_map_tile(
        &mut self,
        index: &GfxType,
        pos: (u32, u32, u32),
        tile: TileData,
    ) {
        if let GfxType::Map(gfx_index) = index {
            if let Some(gfx) = self.map_storage.get_mut(*gfx_index) {
                gfx.gfx.set_tile(pos, tile);
            }
        }
    }

    pub fn add_area_light(
        &mut self,
        index: &GfxType,
        light: AreaLight,
    ) -> Option<Index> {
        if let GfxType::Light(gfx_index) = index {
            if let Some(gfx) = self.light_storage.get_mut(*gfx_index) {
                return gfx.gfx.insert_area_light(light);
            }
        }
        None
    }

    pub fn add_directional_light(
        &mut self,
        index: &GfxType,
        light: DirectionalLight,
    ) -> Option<Index> {
        if let GfxType::Light(gfx_index) = index {
            if let Some(gfx) = self.light_storage.get_mut(*gfx_index) {
                return gfx.gfx.insert_directional_light(light);
            }
        }
        None
    }

    pub fn remove_area_light(&mut self, index: &GfxType, light_key: Index) {
        if let GfxType::Light(gfx_index) = index {
            if let Some(gfx) = self.light_storage.get_mut(*gfx_index) {
                gfx.gfx.remove_area_light(light_key);
            }
        }
    }

    pub fn remove_directional_light(
        &mut self,
        index: &GfxType,
        light_key: Index,
    ) {
        if let GfxType::Light(gfx_index) = index {
            if let Some(gfx) = self.light_storage.get_mut(*gfx_index) {
                gfx.gfx.remove_directional_light(light_key);
            }
        }
    }

    pub fn set_area_light_pos(
        &mut self,
        index: &GfxType,
        light_key: Index,
        pos: Vec2,
    ) {
        if let GfxType::Light(gfx_index) = index {
            if let Some(gfx) = self.light_storage.get_mut(*gfx_index) {
                if let Some(light_data) = gfx.gfx.get_mut_area_light(light_key)
                {
                    light_data.pos = pos;
                }
            }
        }
    }

    pub fn set_directional_light_pos(
        &mut self,
        index: &GfxType,
        light_key: Index,
        pos: Vec2,
    ) {
        if let GfxType::Light(gfx_index) = index {
            if let Some(gfx) = self.light_storage.get_mut(*gfx_index) {
                if let Some(light_data) =
                    gfx.gfx.get_mut_directional_light(light_key)
                {
                    light_data.pos = pos;
                }
            }
        }
    }
}
