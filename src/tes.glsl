#version 410 core

layout (triangles, equal_spacing, cw) in;

out vec4 vs_color;

void main(void) {
  gl_Position = (
    gl_TessCoord.x * gl_in[0].gl_Position +
    gl_TessCoord.y * gl_in[1].gl_Position +
    gl_TessCoord.z * gl_in[2].gl_Position
  );

  float percent_right = (gl_Position.x + 0.25) / 0.5;

  vs_color = vec4(percent_right, 0.0, 0.0, 1.0);
}
