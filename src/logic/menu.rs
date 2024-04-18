use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::logic::edit::EditLogic;
use crate::logic::play::PlayLogic;
use crate::sprint_the_game::{State, StateTransition};

pub struct LevelButton {
    pub id: u32,
    pub position: (u32, u32),
}

pub struct MenuLogic {
    pub level_buttons: Vec<LevelButton>,
}

impl MenuLogic {
    pub fn new() -> Self {
        return Self {
            level_buttons: Vec::from([LevelButton {
                id: 0,
                position: (25, 25),
            }, LevelButton {
                id: 1,
                position: (300, 300),
            }])
        };
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