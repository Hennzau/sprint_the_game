use wgpu::{Queue, RenderPass};
use crate::logic::play::PlayLogic;

pub struct PlayRenderer {}

impl PlayRenderer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn update(&mut self, logic: &PlayLogic) {}
    pub fn process_resize(&mut self, (width, height): (u32, u32),queue: &Queue) {}
    pub fn render<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>) {}
}