use wgpu::{Device, MultisampleState, Queue, RenderPass, SurfaceConfiguration};
use wgpu_text::{BrushBuilder, TextBrush};
use wgpu_text::glyph_brush::ab_glyph::FontRef;
use wgpu_text::glyph_brush::{BuiltInLineBreaker, Layout, Section, Text};

use crate::logic::menu::MenuLogic;

pub struct MenuRenderer {
    brush: TextBrush<FontRef<'static>>,
}

impl MenuRenderer {
    pub fn new(device: &Device, queue: &Queue, config: &SurfaceConfiguration) -> Self {
        let font: &[u8] = include_bytes!("fonts/BulletTrace7-rppO.ttf");
        let mut brush = BrushBuilder::using_font_bytes(font).unwrap().build(
            &device,
            config.width,
            config.height,
            config.format,
        );
        let text = "SPRINT THE GAME".to_string();

        let section = Section::default()
            .add_text(
                Text::new(&text)
                    .with_scale(50.0)
                    .with_color([0.9, 0.5, 0.5, 1.0]),
            )
            .with_bounds((config.width as f32, config.height as f32))
            .with_layout(
                Layout::default()
                    .line_breaker(BuiltInLineBreaker::AnyCharLineBreaker),
            );

        match brush.queue(&device, &queue, vec![&section]) {
            Ok(_) => (),
            Err(err) => {
                panic!("{err}");
            }
        };

        return Self {
            brush
        };
    }

    pub fn update(&mut self, logic: &MenuLogic) {}
    pub fn process_resize(&mut self, (width, height): (u32, u32), queue: &Queue) {
        self.brush.resize_view(width as f32, height as f32, &queue);
    }
    pub fn render<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>) {
        self.brush.draw(render_pass);
    }
}