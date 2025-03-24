

#import bevy_pbr::{
    mesh_view_bindings::view,
    forward_io::VertexOutput,
    utils::coords_to_viewport_uv,
}

struct WaterMaterial {
    shallow_color: vec4<f32>,
    deep_color: vec4<f32>,
    edge_color: vec4<f32>,
    edge_scale: f32,
    water_clarity: f32,
};

@group(1) @binding(0)
var<uniform> material: WaterMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(

    mesh: MeshVertexOutput,
// #ifdef MULTISAMPLED
//     @builtin(sample_index) sample_index: u32,
// #endif
) -> @location(0) vec4<f32> {
    let sample_index = 0u;
    let depth_ndc = bevy_pbr::prepass_utils::prepass_depth(mesh.position, sample_index);
    let depth_buffer_view = ndc_depth_to_linear(depth_ndc);
    let fragment_view = ndc_depth_to_linear(mesh.position.z);    
    let depth_diff_view = fragment_view - depth_buffer_view;
    let beers_law = exp(-depth_diff_view * material.water_clarity);

    let depth_color = vec4<f32>(mix(material.deep_color.xyz, material.shallow_color.xyz, beers_law), 1.0 - beers_law);
    let water_color = mix(material.edge_color, depth_color, smoothstep(0.0, material.edge_scale, depth_diff_view));

    return water_color;
    //return vec4(depth_ndc, depth_ndc, depth_ndc, 1.0);
    //return material.color * textureSample(base_color_texture, base_color_sampler, mesh.uv);
}


fn ndc_depth_to_linear(ndc_depth: f32) -> f32 {
  return -view.projection[3][2] / ndc_depth;
}

// @fragment
// fn fragment(

//     mesh: MeshVertexOutput,
// ) -> @location(0) vec4<f32> {

// //#ifndef MULTISAMPLED
//     let sample_index = 0u;
// //#endif
//     let depth = bevy_pbr::prepass_utils::prepass_depth(mesh.position, sample_index);
//     return vec4(depth, depth, depth, 1.0);
//     //return material.color * textureSample(base_color_texture, base_color_sampler, mesh.uv);
// }


// @fragment
// fn fragment(
// #ifdef MULTISAMPLED
//     @builtin(sample_index) sample_index: u32,
// #endif
//     mesh: MeshVertexOutput,
// ) -> @location(0) vec4<f32> {
// #ifndef MULTISAMPLED
//     let sample_index = 0u;
// #endif
//     if settings.show_depth == 1u {
//         let depth = bevy_pbr::prepass_utils::prepass_depth(mesh.position, sample_index);
//         return vec4(depth, depth, depth, 1.0);
//     } else if settings.show_normals == 1u {
//         let normal = bevy_pbr::prepass_utils::prepass_normal(mesh.position, sample_index);
//         return vec4(normal, 1.0);
//     } else if settings.show_motion_vectors == 1u {
//         let motion_vector = bevy_pbr::prepass_utils::prepass_motion_vector(mesh.position, sample_index);
//         return vec4(motion_vector / globals.delta_time, 0.0, 1.0);
//     }

//     return vec4(0.0);
// }
