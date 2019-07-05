precision mediump float;

uniform vec2 resolution;
uniform float time;
uniform sampler2D tex;

void main() {
    vec2 uv = gl_FragCoord.xy / resolution;
    float color = 0.0;

    float c = texture2D(tex, uv).r;
    gl_FragColor = vec4(c, c, c, 1.0);
}