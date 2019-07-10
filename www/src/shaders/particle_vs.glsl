uniform mat4 u_viewProjection;
uniform float u_particleSize;

attribute vec2 texcoord;
attribute vec4 position;
attribute vec2 instancePosition;
varying vec2 v_uv;

void main() {
    vec4 finalPosition = position;
    finalPosition.xy *= u_particleSize;
    finalPosition.xy += instancePosition.xy;

    v_uv = texcoord;

    gl_Position = u_viewProjection * finalPosition;
}