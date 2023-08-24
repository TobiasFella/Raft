#version 150

in vec3 v_position;
in vec2 v_tex_coords;

uniform sampler2D tex;

void main() {
    gl_FragColor = vec4(texture(tex, v_tex_coords).xyz, 1.0);
}
