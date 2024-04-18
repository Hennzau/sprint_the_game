use crate::renderer::ColorVertex;
use crate::renderer::utils::palette::{BLACK, DARKBLUE, INCREASED_DARKBLUE, IVORY};
use crate::renderer::utils::quad::draw_color_quad;

pub mod palette;
pub mod quad;
pub mod pipeline;

pub fn draw_text_box(vertices: &mut Vec<ColorVertex>, indices: &mut Vec<u16>, (left, up): (u32, u32), (width, height): (u32, u32)) {
    draw_color_quad(vertices, indices, (left as u32 - 25, up as u32 - 25), (width as u32 + 50, height as u32 + 50), (IVORY.0, IVORY.1, IVORY.2, 255));
    draw_color_quad(vertices, indices, (left as u32 - 23, up as u32 - 23), (width as u32 + 46, height as u32 + 46), (INCREASED_DARKBLUE.0, INCREASED_DARKBLUE.1, INCREASED_DARKBLUE.2, 255));
    draw_color_quad(vertices, indices, (left as u32 - 18, up as u32 - 18), (width as u32 + 36, height as u32 + 36), (IVORY.0, IVORY.1, IVORY.2, 255));
    draw_color_quad(vertices, indices, (left as u32 - 16, up as u32 - 16), (width as u32 + 32, height as u32 + 32), (BLACK.0, BLACK.1, BLACK.2, 255));
}