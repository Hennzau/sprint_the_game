use wgpu::RenderPass;
use crate::logic::menu::MenuLogic;
use crate::sprint_the_game::State;

pub struct MenuRenderer {}

impl MenuRenderer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn update (&mut self, logic: &MenuLogic) {}
    pub fn render<'pass>(&self, render_pass: &'pass mut RenderPass) {

    }
}