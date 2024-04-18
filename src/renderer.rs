use std::rc::Rc;
use bytemuck::{Pod, Zeroable};
use wgpu::{Adapter, Device, LoadOp, Operations, Queue, RenderPassColorAttachment, RenderPassDescriptor, Surface, SurfaceConfiguration};

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
use crate::renderer::utils::palette::{BLACK, INCREASED_DARKBLUE};
use crate::renderer::utils::pipeline::{ColorPipeline, TexturePipeline};

use crate::sprint_the_game::State;

pub mod menu;
pub mod play;
pub mod victory;
pub mod edit;

pub mod utils;


#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ColorVertex {
    position: [f32; 2],
    color: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TextureVertex {
    position: [f32; 2],
    texture: [f32; 2],
}

pub struct Renderer {
    color: Rc<ColorPipeline>,
    texture: Rc<TexturePipeline>,

    pub menu: MenuRenderer,
    pub play: PlayRenderer,
    pub victory: VictoryRenderer,
    pub edit: EditRenderer,
}

impl Renderer {
    pub fn new(menu: &MenuLogic, play: &PlayLogic, victory: &VictoryLogic, edit: &EditLogic, device: &Device, surface: &Surface, adapter: &Adapter, queue: &Queue, config: &SurfaceConfiguration) -> Self {
        let color = Rc::new(ColorPipeline::new(device, surface, adapter, config));
        let texture = Rc::new(TexturePipeline::new(device, surface, adapter, config));
        let menu = MenuRenderer::new(menu, color.clone(), texture.clone(), device, queue, config);
        let play = PlayRenderer::new(play, color.clone(), texture.clone(), device, surface, adapter, config);
        let victory = VictoryRenderer::new(victory, color.clone(), texture.clone());
        let edit = EditRenderer::new(edit, color.clone(), texture.clone());

        return Self {
            color,
            texture,
            menu,
            play,
            victory,
            edit,
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
                        load: LoadOp::Clear(wgpu::Color {
                            r: BLACK.0 as f64 / 255.0,
                            g: BLACK.1 as f64 / 255.0,
                            b: BLACK.2 as f64 / 255.0,
                            a: 1.0,
                        }),
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

