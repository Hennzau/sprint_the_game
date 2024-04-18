use std::rc::Rc;
use glam::Mat4;
use wgpu::{BindGroup, Buffer, Device, MultisampleState, Queue, RenderPass, SurfaceConfiguration};
use wgpu::util::DeviceExt;
use wgpu_text::glyph_brush::ab_glyph::FontRef;
use wgpu_text::{BrushBuilder, TextBrush};
use wgpu_text::glyph_brush::{BuiltInLineBreaker, Layout, Section, Text};
use winit::dpi::{LogicalPosition, PhysicalPosition};

use crate::logic::menu::MenuLogic;
use crate::renderer::ColorVertex;
use crate::renderer::utils::draw_text_box;
use crate::renderer::utils::palette::{BLACK, DARKBLUE, IVORY};
use crate::renderer::utils::pipeline::{ColorPipeline, TexturePipeline};
use crate::renderer::utils::quad::draw_color_quad;

pub struct MenuRenderer {
    brush: TextBrush<FontRef<'static>>,
    projection_view_model_uniform: Buffer,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    indices_count: usize,
    bind_group: BindGroup,

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

        draw_text_box(&mut vertex_data, &mut index_data, (left as u32, up as u32), (width as u32, height as u32));
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

        return Self {
            brush,
            projection_view_model_uniform,
            vertex_buffer,
            index_buffer,
            indices_count,
            bind_group,
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
        render_pass.set_pipeline(&self.color.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.indices_count as u32, 0, 0..1);

        self.brush.draw(render_pass);
    }
}