use std::borrow::Cow;
use std::mem;
use glam::Mat4;
use wgpu::{Adapter, BindGroup, Buffer, Device, Face, Queue, RenderPass, RenderPipeline, ShaderModule, Surface, SurfaceConfiguration};
use wgpu::util::DeviceExt;
use crate::logic::play::PlayLogic;
use crate::renderer::ColorVertex;
use crate::renderer::palette::{BLACK, IVORY};
use crate::renderer::quad::draw_color_quad;

mod level;

pub struct PlayRenderer {
    pub bind_group: BindGroup,
    pub shader: ShaderModule,
    pub render_pipeline: RenderPipeline,

    aspect_ratio: f32,
    projection_view_uniform: Buffer,

    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,

    pub index_count: usize,
}

impl PlayRenderer {
    pub fn build_pipeline(device: &Device, surface: &Surface, adapter: &Adapter, config: &SurfaceConfiguration) -> (BindGroup, ShaderModule, RenderPipeline, Buffer) {
        let projection_view_data = Mat4::orthographic_rh(0f32, config.width as f32, config.height as f32, 0f32, -1.0, 1.0);
        let projection_view_ref: &[f32; 16] = projection_view_data.as_ref();
        let projection_view_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(projection_view_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout for Play Renderer"),
            entries: &[
                wgpu::BindGroupLayoutEntry { // Projection * View Matrix
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                }
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("BindGroup for Terrain Renderer"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: projection_view_uniform.as_entire_binding(),
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/color.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];
        let vertex_size = mem::size_of::<ColorVertex>();

        let buffer_layout = wgpu::VertexBufferLayout {
            array_stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Unorm8x4,
                    offset: 2 * 4,
                    shader_location: 1,
                }
            ],
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(Face::Front),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        return (bind_group, shader, render_pipeline, projection_view_uniform);
    }
    pub fn new(logic: &PlayLogic, device: &Device, surface: &Surface, adapter: &Adapter, config: &SurfaceConfiguration) -> Self {
        let (bind_group, shader, render_pipeline, projection_view_uniform) = Self::build_pipeline(device, surface, adapter, config);

        let mut vertex_data: Vec<ColorVertex> = Vec::new();
        let mut index_data: Vec<u16> = Vec::new();

        draw_color_quad(&mut vertex_data, &mut index_data, (0, 0), (400, 400), (IVORY.0, IVORY.1, IVORY.2, 255));
        draw_color_quad(&mut vertex_data, &mut index_data, (2, 2), (400 - 4, 400 - 4), (BLACK.0, BLACK.1, BLACK.2, 255));
        draw_color_quad(&mut vertex_data, &mut index_data, (4, 4), (400 - 8, 400 - 8), (IVORY.0, IVORY.1, IVORY.2, 255));

        let index_count = index_data.len();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        return Self {
            bind_group,
            shader,
            render_pipeline,

            projection_view_uniform,
            aspect_ratio: config.width as f32 / config.height as f32,
            vertex_buffer,
            index_buffer,
            index_count,
        };
    }

    pub fn update(&mut self, logic: &PlayLogic) {}

    pub fn process_resize(&mut self, (width, height): (u32, u32), queue: &Queue) {
        self.aspect_ratio = width as f32 / height as f32;

        let projection_view_data = Mat4::orthographic_rh(0f32, width as f32, height as f32, 0f32, -1.0, 1.0);
        let projection_view_ref: &[f32; 16] = projection_view_data.as_ref();

        queue.write_buffer(&self.projection_view_uniform, 0, bytemuck::cast_slice(projection_view_ref));
    }

    pub fn render<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.index_count as u32, 0, 0..1);
    }
}