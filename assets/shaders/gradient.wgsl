#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct GradientColors {
    bottom: vec4<f32>,
    top: vec4<f32>,
}

@group(2) @binding(0) 
var<uniform> colors: GradientColors;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return mix(colors.bottom, colors.top, in.uv.y);
}

