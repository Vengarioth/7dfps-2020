//VERTEX_SHADER_BEGIN
#version 450
layout(location = 0) in vec3 v_position;
layout(location = 1) in vec3 v_normal;
layout(location = 2) in vec2 v_uv;

layout(location = 0) out vec3 f_position;
layout(location = 1) out vec3 f_normal;
layout(location = 2) out vec2 f_uv;

layout(set = 0, binding = 0) uniform Camera {mat4 view_proj;};
layout(set = 1, binding = 0) uniform Transform {mat4 model;};

void main() {
    //copied from bevy forward.vert
    f_normal = mat3(model) * v_normal;
    f_position = (model * vec4(v_position, 1.0)).xyz;
    f_uv = v_uv;
    gl_Position = view_proj * vec4(v_position, 1.0);
}

//FRAGMENT_SHADER_BEGIN (this comment is used in the code to split them apart)
#version 450
layout(location = 0) in vec3 f_position;
layout(location = 1) in vec3 f_normal;
layout(location = 2) in vec2 f_uv;

layout(location = 0) out vec4 o_target;
layout(set = 1, binding = 1) uniform PetBodyMaterial_color0 {vec4 color0;};
layout(set = 1, binding = 2) uniform PetBodyMaterial_color1 {vec4 color1;};
layout(set = 1, binding = 2) uniform PetBodyMaterial_color2 {vec4 color2;};

void main() {
    vec4 black = vec4(0.0, 0.0, 0.0, 1.0);

    vec4 sample = texture(sampler2D(
        PetBodyMaterial_texture, 
        PetBodyMaterial_texture_sampler),
        f_uv);

    vec4 albedo = black;
    albedo = mix(albedo, color0, sample.r);
    albedo = mix(albedo, color1, sample.g);
    albedo = mix(albedo, color2, sample.b);
    //TODO: gamma

    float n_dot_l = dot(f_nor, vec3(0., .707, .707)) //the sun shines wherever you are <3
    n_dot_l = max(0., n_dot_l);

    vec4 linear = albedo * mix(0.4, 1.0, n_dot_l);

    o_target = albedo * mix(0.4, 1.0, n_dot_l);
    o_target = vec4(0.8, 0.0, 0.3, 1.0);
}