#version 140

uniform mat4 matrix;
uniform float progress;

in vec2 position;
in float time;
out vec4 vColor;

void main() {
    gl_Position = vec4(position, 0.0, 1.0) * matrix;

    float wait_time = 0.5;
    float fade_speed = 1.0;
    float alpha =  0.05 * clamp((progress - time - wait_time)*fade_speed, 0.0, 1.0);
    vColor = vec4(0.0, 0.0, 0.0, alpha);
}