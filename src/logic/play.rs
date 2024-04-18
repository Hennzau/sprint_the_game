use winit::event::{ElementState, KeyEvent, MouseButton};
use crate::logic::victory::VictoryLogic;
use crate::sprint_the_game::StateTransition;

pub mod level;

pub struct PlayLogic {}

impl PlayLogic {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn get_current_level_id(&self) -> u32 {
        return 0;
    }

    pub fn set_current_level_id(&mut self, id: u32) {}

    pub fn reload_current_level(&mut self) {}

    pub fn move_to_next_level(&mut self) {}

    pub fn process_keyboard(&mut self, key_event: KeyEvent) {}

    pub fn process_mouse(&mut self, element_state: ElementState, mouse_button: MouseButton) {}


    pub fn update(&mut self, delta_time: f32, transition: &mut StateTransition, victory: &mut VictoryLogic) {}
}