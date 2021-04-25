varying vec2 v_vt;

#ifdef VERTEX_SHADER
attribute vec2 a_pos;

uniform mat4 u_projection_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_model_matrix;

void main() {
    v_vt = vec2(a_pos.x, 1.0 - a_pos.y);
    gl_Position = u_projection_matrix * u_view_matrix * u_model_matrix * vec4(a_pos, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
uniform sampler2D u_texture;
uniform vec4 u_color;
void main() {
    gl_FragColor = texture2D(u_texture, v_vt) * u_color;
}
#endif