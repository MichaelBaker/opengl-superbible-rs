#version 410 core

layout (triangles) in;

layout (triangle_strip, max_vertices = 9) out;

out vec4 vs_color;

void main(void) {
  int i;

  for (i = 0; i < gl_in.length(); i++) {
    gl_Position = gl_in[i].gl_Position;
    float percent_right = (gl_Position.x + 0.25) / 0.5;
    vs_color = vec4(percent_right, 0.0, 0.0, 1.0);
    EmitVertex();
  }
  EndPrimitive();

  for (i = 0; i < gl_in.length(); i++) {
    gl_Position = gl_in[i].gl_Position + vec4(0.25, 0.0, 0.0, 0.0);
    float percent_right = (gl_Position.x + 0.25) / 0.5;
    vs_color = vec4(0.0, 0.0, percent_right, 1.0);
    EmitVertex();
  }
  EndPrimitive();

  for (i = 0; i < gl_in.length(); i++) {
    gl_Position = gl_in[i].gl_Position + vec4(-0.25, 0.0, 0.0, 0.0);
    float percent_right = (gl_Position.x + 0.25) / 0.5;
    vs_color = vec4(0.0, percent_right, 0.0, 1.0);
    EmitVertex();
  }
  EndPrimitive();

}
