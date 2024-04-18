use crate::renderer::ColorVertex;

pub fn draw_color_quad(vertices: &mut Vec<ColorVertex>, indices: &mut Vec<u16>, position: (u32, u32), (width, height): (u32, u32), color: (u8, u8, u8, u8)) {
    let offset = vertices.len();

    vertices.push(ColorVertex {
        position: [position.0 as f32, position.1 as f32],
        color: [color.0, color.1, color.2, color.3],
    });

    vertices.push(ColorVertex {
        position: [(position.0 + width) as f32, position.1 as f32],
        color: [color.0, color.1, color.2, color.3],
    });

    vertices.push(ColorVertex {
        position: [position.0 as f32, (position.1 + height) as f32],
        color: [color.0, color.1, color.2, color.3],
    });

    vertices.push(ColorVertex {
        position: [(position.0 + width) as f32, (position.1 + height) as f32],
        color: [color.0, color.1, color.2, color.3],
    });

    indices.push(offset as u16);
    indices.push(offset as u16 + 1);
    indices.push(offset as u16 + 2);
    indices.push(offset as u16 + 2);
    indices.push(offset as u16 + 1);
    indices.push(offset as u16 + 3);

}