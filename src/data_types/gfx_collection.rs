use std::borrow::Cow;

use cosmic_text::Attrs;
use graphics::*;
use indexmap::IndexSet;
use slotmap::SlotMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct GfxType(Index);

pub struct GfxData {
    pub layer: usize,
    pub visible: bool,
    pub override_visible: Option<bool>,
    pub debug_track: bool,
    pub identifier: Cow<'static, str>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct LightIndex {
    ltype: u8,
    index: Index,
}

impl LightIndex {
    fn new(ltype: u8, index: Index) -> Self {
        LightIndex { ltype, index }
    }
}

pub struct LightGfx {
    pub light: Lights,
    visible_lights: IndexSet<LightIndex>,
}

pub enum GfxEnum {
    Image(Image),
    Rect(Rect),
    Text(Text),
    Light(LightGfx),
    Mesh(Mesh2D),
}

pub struct Gfx {
    pub data: GfxData,
    pub gfx: GfxEnum,
}

#[derive(Default)]
pub struct GfxCollection {
    pub storage: SlotMap<Index, Gfx>,
}

impl GfxCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_image(
        &mut self,
        mut gfx: Image,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
        view: CameraView,
    ) -> GfxType {
        if view != CameraView::MainView {
            gfx.set_camera_view(view);
        }
        let data = GfxData {
            layer,
            visible,
            override_visible: None,
            identifier: identifier.into(),
            debug_track: false,
        };
        GfxType(self.storage.insert(Gfx {
            data,
            gfx: GfxEnum::Image(gfx),
        }))
    }

    pub fn add_rect(
        &mut self,
        mut gfx: Rect,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
        view: CameraView,
    ) -> GfxType {
        if view != CameraView::MainView {
            gfx.set_camera_view(view);
        }
        let data = GfxData {
            layer,
            visible,
            override_visible: None,
            identifier: identifier.into(),
            debug_track: false,
        };
        GfxType(self.storage.insert(Gfx {
            data,
            gfx: GfxEnum::Rect(gfx),
        }))
    }

    pub fn add_text(
        &mut self,
        mut gfx: Text,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
        view: CameraView,
    ) -> GfxType {
        if view != CameraView::MainView {
            gfx.set_camera_view(view);
        }
        let data = GfxData {
            layer,
            visible,
            override_visible: None,
            identifier: identifier.into(),
            debug_track: false,
        };
        GfxType(self.storage.insert(Gfx {
            data,
            gfx: GfxEnum::Text(gfx),
        }))
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
            override_visible: None,
            identifier: identifier.into(),
            debug_track: false,
        };
        GfxType(self.storage.insert(Gfx {
            data,
            gfx: GfxEnum::Light(LightGfx {
                light: gfx,
                visible_lights: IndexSet::default(),
            }),
        }))
    }

    pub fn add_mesh(
        &mut self,
        gfx: Mesh2D,
        layer: usize,
        identifier: impl Into<Cow<'static, str>>,
        visible: bool,
    ) -> GfxType {
        let data = GfxData {
            layer,
            visible,
            override_visible: None,
            identifier: identifier.into(),
            debug_track: false,
        };
        GfxType(self.storage.insert(Gfx {
            data,
            gfx: GfxEnum::Mesh(gfx),
        }))
    }

    pub fn clear_mesh(&mut self, index: &GfxType) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Mesh(gfx) = &mut data.gfx
        {
            gfx.clear();
        }
    }

    pub fn update_mesh_builder(
        &mut self,
        index: &GfxType,
        builder: &Mesh2DBuilder,
    ) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Mesh(gfx) = &mut data.gfx
        {
            gfx.from_builder(builder);
        }
    }

    pub fn remove_gfx(&mut self, renderer: &mut GpuRenderer, index: &GfxType) {
        if let Some(data) = self.storage.remove(index.0) {
            match data.gfx {
                GfxEnum::Image(gfx) => gfx.unload(renderer),
                GfxEnum::Rect(gfx) => gfx.unload(renderer),
                GfxEnum::Text(gfx) => gfx.unload(renderer),
                GfxEnum::Light(gfx) => gfx.light.unload(renderer),
                GfxEnum::Mesh(gfx) => gfx.unload(renderer),
            }
        }
        self.storage.remove(index.0);
    }

    pub fn get_visible(&mut self, index: &GfxType) -> bool {
        if let Some(data) = self.storage.get(index.0) {
            return data.data.visible;
        }

        false
    }

    pub fn set_visible(&mut self, index: &GfxType, visible: bool) {
        if let Some(data) = self.storage.get_mut(index.0) {
            data.data.visible = visible;

            match &mut data.gfx {
                GfxEnum::Image(gfx) => gfx.changed = true,
                GfxEnum::Rect(gfx) => gfx.changed = true,
                GfxEnum::Text(gfx) => gfx.changed = true,
                GfxEnum::Light(gfx) => gfx.light.changed = true,
                GfxEnum::Mesh(gfx) => gfx.changed = true,
            }
        }
    }

    pub fn set_override_visible(
        &mut self,
        index: &GfxType,
        visible: Option<bool>,
    ) {
        if let Some(data) = self.storage.get_mut(index.0) {
            data.data.override_visible = visible;

            match &mut data.gfx {
                GfxEnum::Image(gfx) => gfx.changed = true,
                GfxEnum::Rect(gfx) => gfx.changed = true,
                GfxEnum::Text(gfx) => gfx.changed = true,
                GfxEnum::Light(gfx) => gfx.light.changed = true,
                GfxEnum::Mesh(gfx) => gfx.changed = true,
            }
        }
    }

    pub fn set_debug(&mut self, index: &GfxType) {
        if let Some(data) = self.storage.get_mut(index.0) {
            data.data.debug_track = true;
        }
    }

    pub fn set_image(&mut self, index: &GfxType, texture: usize) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Image(gfx) = &mut data.gfx
        {
            gfx.texture = Some(texture);
            gfx.changed = true;
        }
    }

    pub fn set_color(&mut self, index: &GfxType, color: Color) {
        if let Some(data) = self.storage.get_mut(index.0) {
            match &mut data.gfx {
                GfxEnum::Image(gfx) => {
                    gfx.set_color(color);
                }
                GfxEnum::Rect(gfx) => {
                    gfx.set_color(color);
                }
                GfxEnum::Text(gfx) => {
                    gfx.set_default_color(color);
                }
                _ => {}
            }
        }
    }

    pub fn set_border_color(&mut self, index: &GfxType, color: Color) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Rect(gfx) = &mut data.gfx
        {
            gfx.set_border_color(color);
        }
    }

    pub fn set_border_width(&mut self, index: &GfxType, width: f32) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Rect(gfx) = &mut data.gfx
        {
            gfx.set_border_width(width);
        }
    }

    pub fn set_pos_z(&mut self, index: &GfxType, z: f32) {
        let mut pos = self.get_pos(index);
        pos.z = z;
        self.set_pos(index, pos);
    }

    pub fn set_pos(&mut self, index: &GfxType, pos: Vec3) {
        if let Some(data) = self.storage.get_mut(index.0) {
            match &mut data.gfx {
                GfxEnum::Image(gfx) => {
                    gfx.pos = pos;
                    gfx.changed = true;
                }
                GfxEnum::Rect(gfx) => {
                    if gfx.pos == pos {
                        return;
                    }
                    gfx.set_pos(pos);
                }
                GfxEnum::Text(gfx) => {
                    if gfx.pos == pos {
                        return;
                    }
                    gfx.set_pos(pos);
                }
                GfxEnum::Light(gfx) => {
                    if gfx.light.pos == pos {
                        return;
                    }
                    gfx.light.set_pos(pos);
                }
                GfxEnum::Mesh(gfx) => {
                    if gfx.pos == pos {
                        return;
                    }
                    gfx.set_pos(pos);
                }
            }
        }
    }

    pub fn set_override_pos(&mut self, index: &GfxType, pos: Vec3) {
        if let Some(data) = self.storage.get_mut(index.0) {
            match &mut data.gfx {
                GfxEnum::Image(gfx) => {
                    gfx.set_order_override(pos);
                }
                GfxEnum::Rect(gfx) => {
                    gfx.set_order_pos(pos);
                }
                GfxEnum::Text(gfx) => {
                    gfx.set_order_override(pos);
                }
                GfxEnum::Mesh(gfx) => {
                    gfx.set_order_pos(pos);
                }
                _ => {}
            }
        }
    }

    pub fn set_render_layer(&mut self, index: &GfxType, render_layer: u32) {
        if let Some(data) = self.storage.get_mut(index.0) {
            match &mut data.gfx {
                GfxEnum::Image(gfx) => {
                    gfx.set_order_layer(render_layer);
                }
                GfxEnum::Rect(gfx) => {
                    gfx.set_order_layer(render_layer);
                }
                GfxEnum::Text(gfx) => {
                    gfx.set_order_layer(render_layer);
                }
                GfxEnum::Mesh(gfx) => {
                    gfx.set_order_layer(render_layer);
                }
                _ => {}
            }
        }
    }

    pub fn set_bound(&mut self, index: &GfxType, bound: Option<Bounds>) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Text(gfx) = &mut data.gfx
        {
            gfx.set_bounds(bound);
        }
    }

    pub fn set_size(&mut self, index: &GfxType, size: Vec2) {
        if let Some(data) = self.storage.get_mut(index.0) {
            match &mut data.gfx {
                GfxEnum::Image(gfx) => {
                    gfx.set_size(size);
                }
                GfxEnum::Rect(gfx) => {
                    gfx.set_size(size);
                }
                GfxEnum::Text(gfx) => {
                    gfx.size = size;
                    gfx.changed = true;
                }
                GfxEnum::Light(gfx) => {
                    gfx.light.set_size(size);
                }
                _ => {}
            }
        }
    }

    pub fn set_buffer_size(
        &mut self,
        renderer: &mut GpuRenderer,
        index: &GfxType,
        size: Vec2,
    ) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Text(gfx) = &mut data.gfx
        {
            gfx.set_buffer_size(renderer, Some(size.x), Some(size.y));
        }
    }

    pub fn set_uv(&mut self, index: &GfxType, uv: Vec4) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Image(gfx) = &mut data.gfx
        {
            let _ = gfx.set_uv(uv);
        }
    }

    pub fn set_text(
        &mut self,
        renderer: &mut GpuRenderer,
        index: &GfxType,
        msg: &str,
    ) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Text(gfx) = &mut data.gfx
        {
            gfx.set_text(renderer, msg, &Attrs::new(), Shaping::Advanced, None);
        }
    }

    pub fn set_custom_text(
        &mut self,
        renderer: &mut GpuRenderer,
        index: &GfxType,
        msg: &str,
        attrs: Attrs,
    ) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Text(gfx) = &mut data.gfx
        {
            gfx.set_text(renderer, msg, &attrs, Shaping::Advanced, None);
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
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Text(gfx) = &mut data.gfx
        {
            gfx.set_rich_text(
                renderer,
                msg,
                &Attrs::new(),
                Shaping::Advanced,
                None,
            );
        }
    }

    pub fn set_text_wrap(
        &mut self,
        renderer: &mut GpuRenderer,
        index: &GfxType,
        can_wrap: bool,
    ) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Text(gfx) = &mut data.gfx
        {
            gfx.set_wrap(
                renderer,
                if can_wrap {
                    cosmic_text::Wrap::Word
                } else {
                    cosmic_text::Wrap::None
                },
            );
        }
    }

    pub fn center_text(&mut self, index: &GfxType) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Text(gfx) = &mut data.gfx
            && let Some(bound) = gfx.bounds
        {
            let size = gfx.measure();
            let textbox_size = bound.right - bound.left;
            gfx.pos.x = bound.left + ((textbox_size * 0.5) - (size.x * 0.5));
            gfx.changed = true;
        }
    }

    pub fn get_pos_and_size(&self, index: &GfxType) -> (Vec2, Vec2) {
        if let Some(data) = self.storage.get(index.0) {
            match &data.gfx {
                GfxEnum::Image(gfx) => {
                    return (Vec2::new(gfx.pos.x, gfx.pos.y), gfx.size);
                }
                GfxEnum::Rect(gfx) => {
                    return (Vec2::new(gfx.pos.x, gfx.pos.y), gfx.size);
                }
                GfxEnum::Text(gfx) => {
                    return (Vec2::new(gfx.pos.x, gfx.pos.y), gfx.size);
                }
                _ => {}
            }
        }

        (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0))
    }

    pub fn get_pos(&mut self, index: &GfxType) -> Vec3 {
        if let Some(data) = self.storage.get(index.0) {
            match &data.gfx {
                GfxEnum::Image(gfx) => {
                    return gfx.pos;
                }
                GfxEnum::Rect(gfx) => {
                    return gfx.pos;
                }
                GfxEnum::Text(gfx) => {
                    return gfx.pos;
                }
                _ => {}
            }
        }

        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn get_size(&self, index: &GfxType) -> Vec2 {
        if let Some(data) = self.storage.get(index.0) {
            match &data.gfx {
                GfxEnum::Image(gfx) => {
                    return gfx.size;
                }
                GfxEnum::Rect(gfx) => {
                    return gfx.size;
                }
                GfxEnum::Text(gfx) => {
                    return gfx.size;
                }
                _ => {}
            }
        }

        Vec2::new(0.0, 0.0)
    }

    pub fn get_uv(&mut self, index: &GfxType) -> Vec4 {
        if let Some(data) = self.storage.get(index.0)
            && let GfxEnum::Image(gfx) = &data.gfx
        {
            return gfx.uv;
        }

        Vec4::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn get_color(&self, index: &GfxType) -> Color {
        if let Some(data) = self.storage.get(index.0) {
            match &data.gfx {
                GfxEnum::Image(gfx) => {
                    return gfx.color;
                }
                GfxEnum::Rect(gfx) => {
                    return gfx.color;
                }
                GfxEnum::Text(gfx) => {
                    return gfx.default_color;
                }
                _ => {}
            }
        }

        Color::rgba(0, 0, 0, 0)
    }

    pub fn get_measure(&self, index: &GfxType) -> Vec2 {
        if let Some(data) = self.storage.get(index.0)
            && let GfxEnum::Text(gfx) = &data.gfx
        {
            return gfx.measure();
        }

        Vec2::new(0.0, 0.0)
    }

    pub fn get_override_pos(&self, index: &GfxType) -> DrawOrder {
        if let Some(data) = self.storage.get(index.0) {
            match &data.gfx {
                GfxEnum::Image(gfx) => {
                    return gfx.order;
                }
                GfxEnum::Rect(gfx) => {
                    return gfx.order;
                }
                GfxEnum::Text(gfx) => {
                    return gfx.order;
                }
                GfxEnum::Mesh(gfx) => {
                    return gfx.order;
                }
                _ => {}
            }
        }

        DrawOrder::default()
    }

    pub fn set_light_world_color(&mut self, index: &GfxType, color: Vec4) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
        {
            gfx.light.world_color = color;
            gfx.light.changed = true;
        }
    }

    pub fn count_area_light(&mut self, index: &GfxType) -> usize {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
        {
            gfx.light.area_lights.len()
        } else {
            0
        }
    }

    pub fn count_directional_light(&mut self, index: &GfxType) -> usize {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
        {
            gfx.light.directional_lights.len()
        } else {
            0
        }
    }

    pub fn count_visible_light(&mut self, index: &GfxType) -> usize {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
        {
            gfx.visible_lights.len()
        } else {
            0
        }
    }

    pub fn get_mut_area_light(
        &mut self,
        index: &GfxType,
        light: Index,
    ) -> Option<&mut AreaLight> {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
        {
            return gfx.light.get_mut_area_light(light);
        }

        None
    }

    pub fn add_area_light(
        &mut self,
        index: &GfxType,
        light: AreaLight,
    ) -> Option<Index> {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
        {
            let index = gfx.light.insert_area_light(light);
            if let Some(light_index) = index {
                gfx.visible_lights.insert(LightIndex::new(0, light_index));
            }
            return index;
        }

        None
    }

    pub fn add_directional_light(
        &mut self,
        index: &GfxType,
        light: DirectionalLight,
    ) -> Option<Index> {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
        {
            let index = gfx.light.insert_directional_light(light);
            if let Some(light_index) = index {
                gfx.visible_lights.insert(LightIndex::new(1, light_index));
            }
            return index;
        }

        None
    }

    pub fn remove_area_light(&mut self, index: &GfxType, light_key: Index) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
        {
            gfx.visible_lights
                .swap_remove(&LightIndex::new(0, light_key));
            gfx.light.remove_area_light(light_key);
        }
    }

    pub fn remove_directional_light(
        &mut self,
        index: &GfxType,
        light_key: Index,
    ) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
        {
            gfx.visible_lights
                .swap_remove(&LightIndex::new(1, light_key));
            gfx.light.remove_directional_light(light_key);
        }
    }

    pub fn set_area_light_pos(
        &mut self,
        index: &GfxType,
        light_key: Index,
        pos: Vec2,
        screen_size: Vec2,
    ) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
            && let Some(light_data) = gfx.light.get_mut_area_light(light_key)
        {
            light_data.pos = pos;
            let size = light_data.max_distance;

            let light_index = LightIndex::new(0, light_key);

            if pos.x + size >= 0.0
                && pos.x - size <= screen_size.x
                && pos.y + size >= 0.0
                && pos.y - size <= screen_size.y
            {
                gfx.visible_lights.insert(light_index);
            } else {
                gfx.visible_lights.swap_remove(&light_index);
            }
        }
    }

    pub fn set_directional_light_pos(
        &mut self,
        index: &GfxType,
        light_key: Index,
        pos: Vec2,
        screen_size: Vec2,
    ) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
            && let Some(light_data) =
                gfx.light.get_mut_directional_light(light_key)
        {
            light_data.pos = pos;
            let size = light_data.max_distance;

            let light_index = LightIndex::new(1, light_key);

            if pos.x + size >= 0.0
                && pos.x - size <= screen_size.x
                && pos.y + size >= 0.0
                && pos.y - size <= screen_size.y
            {
                gfx.visible_lights.insert(light_index);
            } else {
                gfx.visible_lights.swap_remove(&light_index);
            }
        }
    }

    pub fn set_area_light_color(
        &mut self,
        index: &GfxType,
        light_key: Index,
        color: Color,
    ) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
            && let Some(light_data) = gfx.light.get_mut_area_light(light_key)
        {
            light_data.color = color;
        }
    }

    pub fn set_directional_light_color(
        &mut self,
        index: &GfxType,
        light_key: Index,
        color: Color,
    ) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
            && let Some(light_data) =
                gfx.light.get_mut_directional_light(light_key)
        {
            light_data.color = color;
        }
    }

    pub fn set_directional_light_dir(
        &mut self,
        index: &GfxType,
        light_key: Index,
        dir: u8,
    ) {
        if let Some(data) = self.storage.get_mut(index.0)
            && let GfxEnum::Light(gfx) = &mut data.gfx
            && let Some(light_data) =
                gfx.light.get_mut_directional_light(light_key)
        {
            light_data.angle = match dir {
                1 => 0.0,   // Right
                2 => 90.0,  // Up
                3 => 180.0, // Left
                _ => 270.0, // Down
            };
        }
    }
}
