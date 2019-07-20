uniform mat4 u_viewProjection;
uniform float u_particleSize;

attribute vec2 texcoord;
attribute vec4 position;

attribute vec2 instancePosition;
attribute vec2 instanceVelocity;
attribute vec3 instanceColor;

varying vec2 v_uv;
varying vec2 v_vel;
varying vec3 v_color;

void main() {
    vec4 finalPosition = position;
    finalPosition.xy *= u_particleSize;
    finalPosition.xy += instancePosition.xy;

    v_uv = texcoord;
    v_vel = instanceVelocity;
    v_color = instanceColor;

    gl_Position = u_viewProjection * finalPosition;
}