use std::cmp::PartialEq;
use wgpu::{Device, Surface, Queue, SurfaceConfiguration, Adapter};
use winit::event::{ElementState, KeyEvent, MouseButton};

use crate::{
    logic::Logic,
    renderer::Renderer,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Menu,
    Play,
    Victory,
    Edit,
}

pub struct StateTransition {
    next_state: State,
}

impl StateTransition {
    pub fn new() -> Self {
        return Self {
            next_state: State::Menu
        };
    }
    pub fn set_next_state(&mut self, state: State) {
        self.next_state = state;
    }
}

pub struct Application {
    state: State,
    transition: StateTransition,

    logic: Logic,
    renderer: Renderer,
}

impl Application {
    pub fn new(device: &Device, surface: &Surface, adapter: &Adapter, queue: &Queue, config: &SurfaceConfiguration) -> Self {
        let state = State::Menu;
        let transition = StateTransition::new();
        let logic = Logic::new();
        let renderer = Renderer::new(&logic.menu, &logic.play, &logic.victory, &logic.edit, device, surface, adapter, queue, config);

        return Self {
            state,
            transition,
            logic,
            renderer,
        };
    }

    pub fn process_keyboard(&mut self, key_event: KeyEvent) {
        self.logic.process_keyboard(&self.state, key_event);
    }

    pub fn process_mouse(&mut self, element_state: ElementState, mouse_button: MouseButton) {
        self.logic.process_mouse(&self.state, element_state, mouse_button);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.logic.update(&self.state, &mut self.transition, delta_time);
        self.renderer.update(&self.state, &self.logic.menu, &self.logic.play, &self.logic.victory, &self.logic.edit);

        self.state = self.transition.next_state;
    }

    pub fn process_resize(&mut self, (width, height): (u32, u32), queue: &Queue) {
        self.renderer.process_resize((width, height), queue);
    }

    pub fn render(&self, device: &Device, surface: &Surface, queue: &Queue) {
        self.renderer.render(&self.state, device, surface, queue);
    }
}