use wgpu::{Queue, RenderPass};
use crate::logic::victory::VictoryLogic;

pub struct VictoryRenderer {}

impl VictoryRenderer {
    pub fn new(logic: &VictoryLogic) -> Self {
        return Self {};
    }

    pub fn update(&mut self, logic: &VictoryLogic) {}
    pub fn process_resize(&mut self, (width, height): (u32, u32),queue: &Queue) {}
    pub fn render<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>) {}
}