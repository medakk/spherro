import * as twgl from "twgl.js";

import { memory } from "spherro/spherro_bg";

import VERTEX_SHADER from './shaders/particle_vs.glsl';
import FRAGMENT_SHADER from './shaders/particle_fs.glsl';

const PARTICLE_SIZE = 100.0;

export default class Renderer {
    constructor(canvas, width, height, particleCount) {
        this.width   = width;
        this.height  = height;

        // Change the fraction to render to a lower resolution
        canvas.width = width / 1;
        canvas.height = height / 1;

        this.gl = canvas.getContext('webgl');

        this.init(this.gl, particleCount);
    }

    init(gl, particleCount) {
        twgl.addExtensionsToContext(gl);

        if (!gl.drawArraysInstanced || !gl.createVertexArray) {
            alert("need drawArraysInstanced and createVertexArray");
            return; //TODO: Graceful exit
        }

        gl.enable(gl.BLEND);
        // gl.blendFuncSeparate(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA,
        //                      gl.ONE, gl.ONE_MINUS_SRC_ALPHA); // Stock blending function
        gl.blendFuncSeparate(gl.SRC_ALPHA, gl.ONE,
                             gl.ONE, gl.ONE);
        gl.clearColor(0,0,0,0);

        const programInfo = twgl.createProgramInfo(gl, [VERTEX_SHADER, FRAGMENT_SHADER]);
        const buffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
        gl.bufferData(gl.ARRAY_BUFFER, particleCount*4*4, gl.DYNAMIC_DRAW);

        const quad = {
            position: [-0.5, -0.5, 0,
                       +0.5, -0.5, 0,
                       -0.5, +0.5, 0,
                       +0.5, +0.5, 0],
            texcoord: [0, 0,
                       1, 0,
                       0, 1,
                       1, 1],
            indices:  [0, 1, 2, 1, 3, 2],
            instancePosition: {
                numComponents: 2,
                buffer: buffer,
                stride: 16,
                offset: 0,
                divisor: 1,
            },
            instanceVelocity: {
                numComponents: 2,
                buffer: buffer,
                stride: 16,
                offset: 8,
                divisor: 1,
            },
        };
        const bufferInfo = twgl.createBufferInfoFromArrays(gl, quad);
        const viewProjection = twgl.m4.ortho(0, this.width, 0, this.height, -1, 1);
        const vertexArrayInfo = twgl.createVertexArrayInfo(gl, programInfo, bufferInfo);

        this.glInfo = {
            programInfo,
            bufferInfo,
            viewProjection,
            vertexArrayInfo,
            buffer: buffer,
        };
    }

    draw(universe, currentTime) {
        const gl = this.gl;
        const size = universe.get_size();
        const stride = universe.get_data_stride() / 4; // 4 bytes a float. TODO: needs more thought
        const particlesPtr = universe.get_data();
        const particles = new Float32Array(memory.buffer, particlesPtr, size * stride);

        //TODO: Get the buffer from rust?
        const particleBuf = new Float32Array(size*4);
        for(var i=0; i<size; i++) {
            particleBuf[i*4 + 0] = particles[i*stride + 0];
            particleBuf[i*4 + 1] = particles[i*stride + 1];
            particleBuf[i*4 + 2] = particles[i*stride + 2];
            particleBuf[i*4 + 3] = particles[i*stride + 3];
        }

        const {programInfo, bufferInfo, vertexArrayInfo, viewProjection, buffer} = this.glInfo;

        gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
        const uniforms = {
            u_particleSize: PARTICLE_SIZE,
            u_time: currentTime * 0.001,
            u_viewProjection: viewProjection,
            u_color: [1.0, 0.0, 0.0, 1.0],
        };
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.useProgram(programInfo.program);
        twgl.setBuffersAndAttributes(gl, programInfo, bufferInfo);
        twgl.setUniforms(programInfo, uniforms);

        //TODO: Understand vertex arrays and their performance implications
        gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
        gl.bufferSubData(gl.ARRAY_BUFFER, 0, particleBuf);
        const vao = vertexArrayInfo.vertexArrayObject;
        gl.bindVertexArray(vao);

        twgl.drawBufferInfo(gl, vertexArrayInfo, gl.TRIANGLES, vertexArrayInfo.numElements, 0, size);
    }
}