use wgpu::{Device, Surface, Queue, SurfaceConfiguration, Adapter};

use crate::{
    logic::Logic,
    renderer::Renderer,
};

pub enum State {
    Menu,
    Play,
    Victory,
    Edit,
}

pub struct Application {
    state: State,

    logic: Logic,
    renderer: Renderer,
}

impl Application {
    pub fn new(device: &Device, surface: &Surface, adapter: &Adapter, queue: &Queue, config: &SurfaceConfiguration) -> Self {
        let state = State::Play;
        let logic = Logic::new();
        let renderer = Renderer::new(&logic.menu, &logic.play, &logic.victory, &logic.edit, device, surface, adapter, queue, config);

        return Self {
            state,
            logic,
            renderer,
        };
    }

    pub fn update(&mut self, delta_time: f32) {
        self.logic.update(&self.state, delta_time);
        self.renderer.update(&self.state, &self.logic.menu, &self.logic.play, &self.logic.victory, &self.logic.edit);
    }

    pub fn process_resize(&mut self, (width, height): (u32, u32), queue: &Queue) {
        self.renderer.process_resize((width, height), queue);
    }

    pub fn render(&self, device: &Device, surface: &Surface, queue: &Queue) {
        self.renderer.render(&self.state, device, surface, queue);
    }
}