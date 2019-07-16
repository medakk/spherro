precision mediump float;

uniform float u_time;
uniform vec4 u_color;

varying vec2 v_uv;
varying vec2 v_vel;

void main() {
    vec2 shifted_uv = v_uv - vec2(0.5, 0.5);

    vec2 vel = min(1.0 + abs(v_vel*0.001), 3.0);
    shifted_uv *= vel.yx;

    float dist2 = dot(shifted_uv, shifted_uv);
    if(dist2 > 0.5*0.5) {
        discard;
    }

    float t = sqrt(dist2) / 0.5;
    vec4 c1 = vec4(0.1, 0.56, 0.7, 1.0);
    vec4 c2 = vec4(0.1, 0.56, 0.7, 0.0);
    gl_FragColor = mix(c1, c2, t);
}