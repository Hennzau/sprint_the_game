use wgpu::{Queue, RenderPass};
use crate::logic::edit::EditLogic;

pub struct EditRenderer {}

impl EditRenderer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn update(&mut self, logic: &EditLogic) {}

    pub fn process_resize(&mut self, (width, height): (u32, u32), queue: &Queue) {}
    pub fn render<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>) {}
}