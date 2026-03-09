#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct PostProcessSettings {
    intensity: f32, // Let's use this to control the "chunkiness"
#ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: vec3<f32>
#endif
}

@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Lower number = bigger pixels.
    // 160.0 to 240.0 is "Extreme/Retro" territory.
    // We use settings.intensity to let you crank it up from your Rust code.
    let base_resolution = 240.0;
    let scale = base_resolution / max(settings.intensity, 0.1);

    // This ensures pixels stay square regardless of screen stretching
    let dimensions = vec2f(textureDimensions(screen_texture));
    let aspect_ratio = dimensions.x / dimensions.y;
    let grid = vec2f(scale * aspect_ratio, scale);

    // Quantize the UVs
    let pixel_uv = floor(in.uv * grid) / grid;

    // Sample the texture
    return textureSample(screen_texture, texture_sampler, pixel_uv);
}
