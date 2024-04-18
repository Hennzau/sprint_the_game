use std::rc::Rc;
use glam::Mat4;
use image::RgbaImage;
use wgpu::{BindGroup, Buffer, Device, MultisampleState, Queue, RenderPass, SurfaceConfiguration, Texture, TextureView};
use wgpu::util::DeviceExt;
use wgpu_text::glyph_brush::ab_glyph::FontRef;
use wgpu_text::{BrushBuilder, TextBrush};
use wgpu_text::glyph_brush::{BuiltInLineBreaker, Layout, Section, Text};
use winit::dpi::{LogicalPosition, PhysicalPosition};

use crate::logic::menu::MenuLogic;
use crate::renderer::{ColorVertex, TextureVertex};
use crate::renderer::utils::draw_text_box;
use crate::renderer::utils::palette::{BLACK, DARKBLUE, IVORY};
use crate::renderer::utils::pipeline::{ColorPipeline, TexturePipeline};
use crate::renderer::utils::quad::draw_texture_quad;

struct Background {
    pub image: RgbaImage,
    pub texture: Texture,
    pub view: TextureView,
    pub bind_group: BindGroup,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub indices_count: usize,
}

pub struct MenuRenderer {
    brush: TextBrush<FontRef<'static>>,
    projection_view_model_uniform: Buffer,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    indices_count: usize,
    bind_group: BindGroup,

    background: Background,

    color: Rc<ColorPipeline>,
    texture: Rc<TexturePipeline>,
}

impl MenuRenderer {
    pub fn new(logic: &MenuLogic, color: Rc<ColorPipeline>, texture: Rc<TexturePipeline>, device: &Device, queue: &Queue, config: &SurfaceConfiguration) -> Self {
        let font: &[u8] = include_bytes!("fonts/BulletTrace7-rppO.ttf");
        let mut brush = BrushBuilder::using_font_bytes(font).unwrap().build(
            &device,
            config.width,
            config.height,
            config.format,
        );

        let text = "SPRINT THE GAME".to_string();

        let mut title = Section::default()
            .add_text(
                Text::new(&text)
                    .with_scale(80.0)
                    .with_color([IVORY.0 as f32 / 255.0, IVORY.1 as f32 / 255.0, IVORY.2 as f32 / 255.0, 1.0]),
            )
            .with_layout(
                Layout::default()
                    .line_breaker(BuiltInLineBreaker::AnyCharLineBreaker),
            );

        let bounds = brush.glyph_bounds(title.clone()).unwrap();
        let title = title.with_screen_position(PhysicalPosition::new(config.width as f32 / 2.0 - bounds.width() / 2.0, config.height as f32 / 4.0 - bounds.height() / 2.0));
        let height = bounds.height();
        let up = config.height as f32 / 4.0 - bounds.height() / 2.0;

        let text = "Select Level to Start".to_string();

        let info = Section::default()
            .add_text(
                Text::new(&text)
                    .with_scale(80.0)
                    .with_color([IVORY.0 as f32 / 255.0, IVORY.1 as f32 / 255.0, IVORY.2 as f32 / 255.0, 1.0]),
            )
            .with_layout(
                Layout::default()
                    .line_breaker(BuiltInLineBreaker::AnyCharLineBreaker),
            );

        let bounds = brush.glyph_bounds(info.clone()).unwrap();
        let info = info.with_screen_position(PhysicalPosition::new(config.width as f32 / 2.0 - bounds.width() / 2.0, config.height as f32 / 4.0 - bounds.height() / 2.0 + height));
        let left = config.width as f32 / 2.0 - bounds.width() / 2.0;
        let width = bounds.width();
        let height = height + bounds.height();

        brush.queue(&device, &queue, vec![&title, &info]).expect("Failed to draw main menu text");

        let projection_view_model_data = Mat4::orthographic_rh(0f32, config.width as f32, config.height as f32, 0f32, -1f32, 1f32);
        let projection_view_model_ref: &[f32; 16] = projection_view_model_data.as_ref();
        let projection_view_model_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(projection_view_model_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Menu BindGroup"),
            layout: &color.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: projection_view_model_uniform.as_entire_binding(),
                },
            ],
        });

        let mut vertex_data: Vec<ColorVertex> = Vec::new();
        let mut index_data: Vec<u16> = Vec::new();

        draw_text_box(&mut vertex_data, &mut index_data, (left as u32, up as u32), (width as u32, height as u32), 15, 15);
        let indices_count = index_data.len();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Menu VertexBuffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Menu IndexBuffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        let diffuse_bytes = include_bytes!("images/Sprint_Background.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();

        use image::GenericImageView;
        let dimensions = diffuse_image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
                view_formats: &[],
            }
        );

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture.layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: projection_view_model_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        let mut vertex_data: Vec<TextureVertex> = Vec::new();
        let mut index_data: Vec<u16> = Vec::new();

        draw_texture_quad(&mut vertex_data, &mut index_data, (0, 0), (config.width, config.height));

        let background_indices_count = index_data.len();

        let background_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Menu VertexBuffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let background_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Menu IndexBuffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        let background = Background {
            image: diffuse_rgba,
            texture: diffuse_texture,
            view: diffuse_texture_view,
            bind_group: diffuse_bind_group,
            vertex_buffer: background_vertex_buffer,
            index_buffer: background_index_buffer,
            indices_count: background_indices_count,
        };

        return Self {
            brush,
            projection_view_model_uniform,
            vertex_buffer,
            index_buffer,
            indices_count,
            bind_group,
            background,
            color,
            texture,
        };
    }

    pub fn update(&mut self, logic: &MenuLogic) {}
    pub fn process_resize(&mut self, (width, height): (u32, u32), queue: &Queue) {
        self.brush.resize_view(width as f32, height as f32, &queue);

        let projection_view_model_data = Mat4::orthographic_rh(0f32, width as f32, height as f32, 0f32, -1f32, 1f32);
        let projection_view_model_ref: &[f32; 16] = projection_view_model_data.as_ref();

        queue.write_buffer(&self.projection_view_model_uniform, 0, bytemuck::cast_slice(projection_view_model_ref));
    }
    pub fn render<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>) {
        render_pass.set_pipeline(&self.texture.pipeline);
        render_pass.set_bind_group(0, &self.background.bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.background.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.background.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.background.indices_count as u32, 0, 0..1);

        render_pass.set_pipeline(&self.color.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.indices_count as u32, 0, 0..1);

        self.brush.draw(render_pass);
    }
}