use winit::event::{ElementState, KeyEvent, MouseButton};
use crate::logic::{
    edit::EditLogic,
    menu::MenuLogic,
    play::PlayLogic,
    victory::VictoryLogic,
};
use crate::sprint_the_game::{State, StateTransition};

pub mod menu;
pub mod play;
pub mod victory;
pub mod edit;

pub struct Logic {
    pub menu: MenuLogic,
    pub play: PlayLogic,
    pub victory: VictoryLogic,
    pub edit: EditLogic,
}

impl Logic {
    pub fn new() -> Self {
        return Self {
            menu: MenuLogic::new(),
            play: PlayLogic::new(),
            victory: VictoryLogic::new(),
            edit: EditLogic::new(),
        };
    }

    pub fn process_keyboard(&mut self, state: &State, key_event: KeyEvent) {
        match state {
            State::Menu => self.menu.process_keyboard(key_event),
            State::Play => self.play.process_keyboard(key_event),
            State::Victory => self.victory.process_keyboard(key_event),
            State::Edit => self.edit.process_keyboard(key_event),
        }
    }

    pub fn process_mouse(&mut self, state: &State, element_state: ElementState, mouse_button: MouseButton) {
        match state {
            State::Menu => self.menu.process_mouse(element_state, mouse_button),
            State::Play => self.play.process_mouse(element_state, mouse_button),
            State::Victory => self.victory.process_mouse(element_state, mouse_button),
            State::Edit => self.edit.process_mouse(element_state, mouse_button),
        }
    }


    pub fn update(&mut self, state: &State, transition: &mut StateTransition, delta_time: f32) {
        match state {
            State::Menu => self.menu.update(delta_time, transition, &mut self.play, &mut self.edit),
            State::Play => self.play.update(delta_time, transition, &mut self.victory),
            State::Victory => self.victory.update(delta_time, transition, &mut self.play),
            State::Edit => self.edit.update(delta_time, transition, &mut self.menu),
        }
    }
}
