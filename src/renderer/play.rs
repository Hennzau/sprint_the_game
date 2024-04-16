use wgpu::RenderPass;
use crate::logic::play::PlayLogic;

pub struct PlayRenderer {}

impl PlayRenderer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn update (&mut self, logic: &PlayLogic) {}
    pub fn render<'pass>(&self, render_pass: &'pass mut RenderPass) {

    }
}