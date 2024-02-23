use crate::{
    AtlasSet, Color, DrawOrder, DrawType, GpuRenderer, ImageVertex, Index,
    OrderedIndex, Vec2, Vec3, Vec4,
};

pub enum TextDrawOrder {
    BelowLights,
    AboveLights,
}
/// rendering data for all images.
pub struct Image {
    pub pos: Vec3,
    pub hw: Vec2,
    // used for static offsets or animation Start positions
    pub uv: Vec4,
    /// Color dah  number / 255.
    pub color: Color,
    // frames, frames_per_row: this will cycle thru
    // frames per rox at the uv start.
    pub frames: Vec2,
    /// in millsecs 1000 = 1sec
    pub switch_time: u32,
    /// turn on animation if set.
    pub animate: bool,
    pub use_camera: bool,
    /// Texture area location in Atlas.
    pub texture: Option<usize>,
    pub store_id: Index,
    pub order: DrawOrder,
    pub render_layer: u32,
    /// if anything got updated we need to update the buffers too.
    pub changed: bool,
}

impl Image {
    pub fn new(
        texture: Option<usize>,
        renderer: &mut GpuRenderer,
        render_layer: u32,
    ) -> Self {
        Self {
            pos: Vec3::default(),
            hw: Vec2::default(),
            uv: Vec4::default(),
            frames: Vec2::default(),
            switch_time: 0,
            animate: false,
            use_camera: true,
            color: Color::rgba(255, 255, 255, 255),
            texture,
            store_id: renderer.new_buffer(),
            order: DrawOrder::default(),
            render_layer,
            changed: true,
        }
    }
    pub fn create_quad(
        &mut self,
        renderer: &mut GpuRenderer,
        atlas: &mut AtlasSet,
    ) {
        let allocation = match &self.texture {
            Some(id) => {
                if let Some(allocation) = atlas.get(*id) {
                    allocation
                } else {
                    return;
                }
            }
            None => return,
        };

        let (u, v, width, height) = allocation.rect();
        let (u, v, width, height) = (
            self.uv.x + u as f32,
            self.uv.y + v as f32,
            self.uv.z.min(width as f32),
            self.uv.w.min(height as f32),
        );

        let instance = ImageVertex {
            position: self.pos.to_array(),
            hw: self.hw.to_array(),
            #[allow(clippy::tuple_array_conversions)]
            tex_data: [u, v, width, height],
            color: self.color.0,
            frames: self.frames.to_array(),
            animate: u32::from(self.animate),
            use_camera: u32::from(self.use_camera),
            time: self.switch_time,
            layer: allocation.layer as i32,
        };

        if let Some(store) = renderer.get_buffer_mut(&self.store_id) {
            store.store = bytemuck::bytes_of(&instance).to_vec();
            store.changed = true;
        }

        self.order = DrawOrder::new(
            self.color.a() < 255,
            &self.pos,
            self.render_layer,
            &self.hw,
            DrawType::Image,
        );
        self.changed = false;
    }

    /// used to check and update the vertex array.
    pub fn update(
        &mut self,
        renderer: &mut GpuRenderer,
        atlas: &mut AtlasSet,
    ) -> OrderedIndex {
        // if pos or tex_pos or color changed.
        if self.changed {
            self.create_quad(renderer, atlas);
        }

        OrderedIndex::new(self.order, self.store_id, 0)
    }
}
