use wgpu::RenderPass;
use crate::logic::victory::VictoryLogic;

pub struct VictoryRenderer {}

impl VictoryRenderer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn update(&mut self, logic: &VictoryLogic) {}
    pub fn render<'pass>(&self, render_pass: &'pass mut RenderPass) {}
}