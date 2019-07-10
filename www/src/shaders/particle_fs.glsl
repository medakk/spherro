precision mediump float;

varying vec2 v_uv;
uniform vec4 u_color;
uniform float u_time;

void main() {
    vec2 center = v_uv + vec2(-0.5, -0.5);
    float dist2 = dot(center, center);
    if(dist2 > 0.5 * 0.5) {
        discard;
    }
    float t = sqrt(dist2) / 0.5;

    vec4 c1 = vec4(0.1, 0.56, 0.7, 1.0);
    vec4 c2 = vec4(0.1, 0.56, 0.7, 0.0);

    gl_FragColor = mix(c1, c2, t);
}