use wgpu::RenderPass;
use crate::logic::edit::EditLogic;

pub struct EditRenderer {}

impl EditRenderer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn update (&mut self, logic: &EditLogic) {}
    pub fn render<'pass>(&self, render_pass: &'pass mut RenderPass) {}
}