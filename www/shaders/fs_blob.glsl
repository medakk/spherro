precision mediump float;

uniform vec2 resolution;
uniform float time;
uniform sampler2D tex;

void main() {
    vec2 uv = gl_FragCoord.xy / resolution;
    float color = 0.0;

    const int R = 2;
    const float d = 0.02;
    const int kernel_size = 2*R+1;
    const float sigma = 1000.0;
    const float A = 0.2;

    float c = 0.0;

    for(int i=-R; i<=R; i++) {
        for(int j=-R; j<=R; j++) {
            vec2 d_uv = uv + d*vec2(i, j);
            float d_c = texture2D(tex, d_uv).r;
            
            vec2 d = d_uv - uv;
            float w = A * exp(-dot(d, d) / sigma);
            c += w*d_c;

        }
    }
    gl_FragColor = vec4(c, c, c, 1.0);
}