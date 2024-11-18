#version 450

layout(location = 0) in vec3 fragmentColors;
layout(location = 0) out vec4 outColor;

void main() {
  outColor = vec4(fragmentColors, 1.0);
}