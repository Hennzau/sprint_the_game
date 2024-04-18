struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location (0) color: vec4<f32>,
};

@group(0)
@binding(0)
var<uniform> projection_view_model_matrix: mat4x4<f32>;

@vertex
fn vs_main(

    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,

) -> VertexOutput {
    var result: VertexOutput;

    result.position = projection_view_model_matrix * vec4<f32> (position.x, position.y, 0.0, 1.0);
    result.color = color;

    return result;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vertex.color;
}