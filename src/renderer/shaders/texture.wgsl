struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) texture_coordinate: vec2<f32>,
};

@group(0)
@binding(0)
var<uniform> projection_view_model_matrix: mat4x4<f32>;

@group(0)
@binding(1)
var texture_image: texture_2d<f32>;

@group(0)
@binding(2)
var texture_sampler: sampler;

@vertex
fn vs_main(

    @location(0) position: vec2<f32>,
    @location(1) texture_coordinate: vec2<f32>,

) -> VertexOutput {
    var result: VertexOutput;

    result.position = projection_view_model_matrix * vec4<f32> (position.x, position.y, 0.0, 1.0);
    result.texture_coordinate = texture_coordinate;

    return result;
}

@fragment
fn fs_main(

    vertex: VertexOutput

    ) -> @location(0) vec4<f32> {
    let color = textureSample(texture_image, texture_sampler, vertex.texture_coordinate);
    // vec4<f32> (pow (color.x, 1.0 / 2.2), pow (color.y, 1.0 / 2.2), pow (color.z, 1.0 / 2.2), pow (color.w, 1.0 / 2.2))
    return color;
}