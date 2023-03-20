#version 450

layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform texture2D CustomMaterial_texture;
layout(set = 1, binding = 1) uniform sampler CustomMaterial_sampler;
layout(set = 1, binding = 2) uniform float time;
layout(set = 1, binding = 3) uniform float intensity;

float pix2 = 3.1415926536 * 2.0;

void main() {
    float ri = (sin(pix2 * time * .35) + 0.5) * intensity;
    float gi = (sin(pix2 * time * .15) + 0.5) * intensity / 2;
    float bi = (sin(pix2 * time * .25) + 0.5) * intensity / 3;
    float x = sin(pix2 * time * .55) * 2.;

    vec2 rOffset = vec2(-0.5 * x * ri, x * ri);
    vec2 gOffset = vec2(2. * gi, x * gi);
    vec2 bOffset = vec2(0., -x * bi);

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
