precision mediump float;

varying vec2 v_uv;
uniform float time;

void main() {
    gl_FragColor = vec4(v_uv.x, v_uv.y, 0.0, 1.0);
}