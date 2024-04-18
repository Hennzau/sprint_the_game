use std::rc::Rc;
use wgpu::{Queue, RenderPass};
use crate::logic::edit::EditLogic;
use crate::renderer::utils::pipeline::{ColorPipeline, TexturePipeline};

pub struct EditRenderer {
    color: Rc<ColorPipeline>,
    texture: Rc<TexturePipeline>,
}

impl EditRenderer {
    pub fn new(logic: &EditLogic, color: Rc<ColorPipeline>, texture: Rc<TexturePipeline>) -> Self {
        return Self {
            color,
            texture
        };
    }

    pub fn update(&mut self, logic: &EditLogic) {}

    pub fn process_resize(&mut self, (width, height): (u32, u32), queue: &Queue) {}
    pub fn render<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>) {}
}