use crate::renderer::ColorVertex;
use crate::renderer::utils::palette::{BLACK, DARKBLUE, INCREASED_DARKBLUE, IVORY};
use crate::renderer::utils::quad::draw_color_quad;

pub mod palette;
pub mod quad;
pub mod pipeline;

pub fn draw_text_box(vertices: &mut Vec<ColorVertex>, indices: &mut Vec<u16>, (left, up): (u32, u32), (width, height): (u32, u32), offset: u32, size: u32) {
    draw_color_quad(vertices, indices, (left - offset - 2 - size - 2, up - offset - 2 - size - 2), (width + 2 * (offset + 2 + size + 2), height + 2 * (offset + 2 + size + 2)), (IVORY.0, IVORY.1, IVORY.2, 255));
    draw_color_quad(vertices, indices, (left - offset - 2 - size, up - offset - 2 - size), (width + 2 * (offset + 2 + size), height + 2 * (offset + 2 + size)), (BLACK.0, BLACK.1, BLACK.2, 255));
    draw_color_quad(vertices, indices, (left - offset - 2, up - offset - 2), (width + 2 * (offset + 2), height + 2 * (offset + 2)), (IVORY.0, IVORY.1, IVORY.2, 255));
    draw_color_quad(vertices, indices, (left - offset, up - offset), (width + 2 * offset, height + 2 * offset), (DARKBLUE.0, DARKBLUE.1, DARKBLUE.2, 255));
}