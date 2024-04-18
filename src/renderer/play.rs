use std::rc::Rc;
use wgpu::{Adapter, BindGroup, Buffer, Device, Face, Queue, RenderPass, RenderPipeline, ShaderModule, Surface, SurfaceConfiguration};
use wgpu::util::DeviceExt;
use crate::logic::play::PlayLogic;
use crate::renderer::utils::pipeline::{ColorPipeline, TexturePipeline};

mod level;

pub struct PlayRenderer {
    color: Rc<ColorPipeline>,
    texture: Rc<TexturePipeline>,
}

impl PlayRenderer {
    pub fn new(logic: &PlayLogic, color: Rc<ColorPipeline>, texture: Rc<TexturePipeline>, device: &Device, surface: &Surface, adapter: &Adapter, config: &SurfaceConfiguration) -> Self {
        return Self {
            color,
            texture
        };
    }

    pub fn update(&mut self, logic: &PlayLogic) {}

    pub fn process_resize(&mut self, (width, height): (u32, u32), queue: &Queue) {}

    pub fn render<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>) {}
}