use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::logic::edit::EditLogic;
use crate::logic::play::PlayLogic;
use crate::sprint_the_game::{State, StateTransition};

pub struct MenuLogic {}

impl MenuLogic {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn process_keyboard(&mut self, key_event: KeyEvent) {
        match key_event {
            KeyEvent {
                physical_key,
                state,
                ..
            } => {
                match physical_key {
                    _ => {}
                }
            }
        }
    }

    pub fn process_mouse(&mut self, element_state: ElementState, mouse_button: MouseButton) {}

    pub fn update(&mut self, delta_time: f32, transition: &mut StateTransition, play: &mut PlayLogic, edit: &mut EditLogic) {}
}