use crate::{
    logic::{
        edit::EditLogic,
        menu::MenuLogic,
        play::PlayLogic,
        victory::VictoryLogic,
    },
    renderer::{
        edit::EditRenderer,
        menu::MenuRenderer,
        play::PlayRenderer,
        victory::VictoryRenderer,
    },
};

pub enum State {
    Menu,
    Play,
    Victory,
    Edit,
}

struct Logic {
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
}

struct Renderer {
    pub menu: MenuRenderer,
    pub play: PlayRenderer,
    pub victory: VictoryRenderer,
    pub edit: EditRenderer,
}

impl Renderer {
    pub fn new() -> Self {
        return Self {
            menu: MenuRenderer::new(),
            play: PlayRenderer::new(),
            victory: VictoryRenderer::new(),
            edit: EditRenderer::new(),
        };
    }
}


pub struct Application {
    state: State,

    logic: Logic,
    renderer: Renderer,
}

impl Application {
    pub fn new() -> Self {
        return Self {
            state: State::Menu,
            logic: Logic::new(),
            renderer: Renderer::new(),
        };
    }

    pub fn update(&mut self, delta_time: f32) {}

    pub fn render(&self) {}
}