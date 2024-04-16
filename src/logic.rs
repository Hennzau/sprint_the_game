use crate::logic::{
    edit::EditLogic,
    menu::MenuLogic,
    play::PlayLogic,
    victory::VictoryLogic,
};
use crate::sprint_the_game::State;

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

    pub fn update(&mut self, state: &State, delta_time: f32) {
        match state {
            State::Menu => self.menu.update(delta_time),
            State::Play => self.play.update(delta_time),
            State::Victory => self.victory.update(delta_time),
            State::Edit => self.edit.update(delta_time),
        }
    }
}
