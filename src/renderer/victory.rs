use std::rc::Rc;
use wgpu::{Queue, RenderPass};
use crate::logic::victory::VictoryLogic;
use crate::renderer::utils::pipeline::{ColorPipeline, TexturePipeline};

pub struct VictoryRenderer {
    color: Rc<ColorPipeline>,
    texture: Rc<TexturePipeline>,
}

impl VictoryRenderer {
    pub fn new(logic: &VictoryLogic, color: Rc<ColorPipeline>, texture: Rc<TexturePipeline>) -> Self {
        return Self {
            color,
            texture
        };
    }

    pub fn update(&mut self, logic: &VictoryLogic) {}
    pub fn process_resize(&mut self, (width, height): (u32, u32), queue: &Queue) {}
    pub fn render<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>) {}
}