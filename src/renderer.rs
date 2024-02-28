use graphics::*;
use winit::dpi::PhysicalSize;

pub mod fade;

pub use fade::*;

use crate::{
    gfx_collection::*, game_content::*, TextureAllocation
};

pub struct DrawSetting {
    pub gfx: GfxCollection,
    pub renderer: GpuRenderer,
    pub size: PhysicalSize<f32>,
    pub scale: f64,
    pub resource: TextureAllocation,
    pub fade: Fade,
}

pub struct State<Controls>
where
    Controls: camera::controls::Controls,
{
    /// World Camera Controls and time. Deturmines how the world is looked at.
    pub system: System<Controls>,
    /// Atlas Groups for Textures in GPU
    pub image_atlas: AtlasSet,
    pub text_atlas: TextAtlas,
    pub map_atlas: AtlasSet,
    pub ui_atlas: AtlasSet,
    /// Rendering Buffers and other shared data.
    pub image_renderer: ImageRenderer,
    pub text_renderer: TextRenderer,
    pub map_renderer: MapRenderer,
    pub ui_renderer: RectRenderer,
}

impl<Controls> Pass for State<Controls>
where
    Controls: camera::controls::Controls,
{
    fn render(
        &mut self,
        renderer: &GpuRenderer,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: renderer.frame_buffer().as_ref().expect("no frame view?"),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.25,
                        b: 0.5,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(
                wgpu::RenderPassDepthStencilAttachment {
                    view: renderer.depth_buffer(),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0),
                        store: wgpu::StoreOp::Store,
                    }),
                },
            ),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Lets set the System's Shader information here, mostly Camera, Size and Time
        pass.set_bind_group(0, self.system.bind_group(), &[]);
        // Lets set the Reusable Vertices and Indicies here.
        // This is used for each Renderer, Should be more performant since it is shared.
        pass.set_vertex_buffer(0, renderer.buffer_object.vertices());
        pass.set_index_buffer(
            renderer.buffer_object.indices(),
            wgpu::IndexFormat::Uint32,
        );

        pass.render_map(renderer, &self.map_renderer, &self.map_atlas, 0);

        pass.render_image(renderer, &self.image_renderer, &self.image_atlas, 0);

        pass.render_map(renderer, &self.map_renderer, &self.map_atlas, 1);

        pass.render_text(renderer, &self.text_renderer, &self.text_atlas, 0);

        pass.render_rects(renderer, &self.ui_renderer, &self.ui_atlas, 0);

        pass.render_text(renderer, &self.text_renderer, &self.text_atlas, 1);

        pass.render_rects(renderer, &self.ui_renderer, &self.ui_atlas, 2);

        pass.render_text(renderer, &self.text_renderer, &self.text_atlas, 2);
    }
}

pub fn add_image_to_buffer<Controls>(
    systems: &mut DrawSetting,
    graphics: &mut State<Controls>,
) where
Controls: camera::controls::Controls, 
{
    systems.gfx.collection.iter_mut().for_each(|data| {
        if data.1.visible {
            match &mut data.1.gfx {
                GfxType::Image(image) => {
                    graphics.image_renderer.image_update(image, &mut systems.renderer, &mut graphics.image_atlas, data.1.layer);
                }
                GfxType::Rect(rect) => {
                    graphics.ui_renderer.rect_update(rect, &mut systems.renderer, &mut graphics.ui_atlas, data.1.layer);
                }
                GfxType::Text(text) => {
                    graphics.text_renderer
                        .text_update(text, &mut graphics.text_atlas, &mut systems.renderer, data.1.layer)
                        .unwrap();
                }
                GfxType::Map(map) => {
                    graphics.map_renderer.map_update(map, &mut systems.renderer, &mut graphics.map_atlas, [0, 1]);
                }
            }
        }
    });
}