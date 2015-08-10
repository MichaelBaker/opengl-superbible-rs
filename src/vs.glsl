#version 410 core

in vec4 offset;

void main(void) {
  gl_Position = offset;
}
