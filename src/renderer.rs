use wgpu::{Device, LoadOp, Operations, Queue, RenderPassColorAttachment, RenderPassDescriptor, Surface, SurfaceConfiguration};

use crate::logic::{
    edit::EditLogic,
    menu::MenuLogic,
    play::PlayLogic,
    victory::VictoryLogic,
};

use crate::renderer::{
    edit::EditRenderer,
    menu::MenuRenderer,
    play::PlayRenderer,
    victory::VictoryRenderer,
};

use crate::sprint_the_game::State;

pub mod menu;
pub mod play;
pub mod victory;
pub mod edit;

pub struct Renderer {
    pub menu: MenuRenderer,
    pub play: PlayRenderer,
    pub victory: VictoryRenderer,
    pub edit: EditRenderer,
}

impl Renderer {
    pub fn new(device: &Device, queue: &Queue, config: &SurfaceConfiguration) -> Self {
        return Self {
            menu: MenuRenderer::new(device, queue, config),
            play: PlayRenderer::new(),
            victory: VictoryRenderer::new(),
            edit: EditRenderer::new(),
        };
    }

    pub fn update(&mut self, state: &State, menu: &MenuLogic, play: &PlayLogic, victory: &VictoryLogic, edit: &EditLogic) {
        match state {
            State::Menu => self.menu.update(menu),
            State::Play => self.play.update(play),
            State::Victory => self.victory.update(victory),
            State::Edit => self.edit.update(edit),
        }
    }

    pub fn process_resize(&mut self, (width, height): (u32, u32), queue: &Queue) {
        self.menu.process_resize((width, height), queue);
        self.play.process_resize((width, height), queue);
        self.victory.process_resize((width, height), queue);
        self.edit.process_resize((width, height), queue);
    }

    pub fn render(&self, state: &State, device: &Device, surface: &Surface, queue: &Queue) {
        let frame = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None,
            });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            match state {
                State::Menu => self.menu.render(&mut pass),
                State::Play => self.play.render(&mut pass),
                State::Victory => self.victory.render(&mut pass),
                State::Edit => self.edit.render(&mut pass),
            }
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    }
}

