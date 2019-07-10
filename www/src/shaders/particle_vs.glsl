uniform mat4 u_viewProjection;
attribute vec2 texcoord;
attribute vec4 position;
attribute vec2 instancePosition;
varying vec2 v_uv;

void main() {
    vec4 finalPosition = position;
    finalPosition.xy += instancePosition.xy;
    v_uv = texcoord;

    gl_Position = u_viewProjection * 400.0 * finalPosition;
}