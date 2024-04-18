use winit::event::{ElementState, KeyEvent, MouseButton};
use crate::logic::menu::MenuLogic;
use crate::sprint_the_game::StateTransition;

pub struct EditLogic {}

impl EditLogic {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn process_keyboard(&mut self, key_event: KeyEvent) {}

    pub fn process_mouse(&mut self, element_state: ElementState, mouse_button: MouseButton) {}

    pub fn update(&mut self, delta_time: f32, transition: &mut StateTransition, menu: &mut MenuLogic) {}
}