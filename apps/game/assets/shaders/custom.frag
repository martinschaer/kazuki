#version 450

layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform texture2D CustomMaterial_texture;
layout(set = 1, binding = 1) uniform sampler CustomMaterial_sampler;

void main() {
    vec2 rOffset = vec2(-0.002, 0.004);
    vec2 gOffset = vec2(0.004, 0.004);
    vec2 bOffset = vec2(0.0, -0.002);

    float rValue = texture(
      sampler2D(CustomMaterial_texture, CustomMaterial_sampler),
      v_Uv + rOffset
    ).r;
    float gValue = texture(
      sampler2D(CustomMaterial_texture, CustomMaterial_sampler),
      v_Uv + gOffset
    ).g;
    float bValue = texture(
      sampler2D(CustomMaterial_texture, CustomMaterial_sampler),
      v_Uv + bOffset
    ).b;

    o_Target = vec4(rValue, gValue, bValue, 1.0);
}
