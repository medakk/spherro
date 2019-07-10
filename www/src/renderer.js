import * as twgl from "twgl.js";

import { memory } from "spherro/spherro_bg";

import VERTEX_SHADER from './shaders/particle_vs.glsl';
import FRAGMENT_SHADER from './shaders/particle_fs.glsl';

const PARTICLE_SIZE = 60.0;

export default class Renderer {
    constructor(canvas, width, height, particleCount) {
        this.width = canvas.width = width;
        this.height = canvas.height = height;
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
                data: new Float32Array(particleCount*2),
                divisor: 1,
            },
        };
        const bufferInfo = twgl.createBufferInfoFromArrays(gl, quad);
        const viewProjection = twgl.m4.ortho(0, this.width, 0, this.height, -1, 1);
        const vertexArrayInfo = twgl.createVertexArrayInfo(gl, programInfo, bufferInfo);

        this.glInfo = {
            programInfo: programInfo,
            bufferInfo: bufferInfo,
            viewProjection: viewProjection,
            vertexArrayInfo: vertexArrayInfo,
        };
    }

    draw(universe, currentTime) {
        const gl = this.gl;
        const cellsPtr = universe.get_data();
        const size = universe.get_size();
        const stride = universe.get_data_stride() / 4; // 4 bytes a float. TODO: needs more thought
        const cells = new Float32Array(memory.buffer, cellsPtr, size * stride);

        //TODO: Get a position buffer from rust
        const cellPositions = new Float32Array(size*2);
        for(var i=0; i<size; i++) {
            cellPositions[i*2+0] = cells[i*stride + 0];
            cellPositions[i*2+1] = cells[i*stride + 1];
        }

        const {programInfo, bufferInfo, vertexArrayInfo, viewProjection} = this.glInfo;

        twgl.resizeCanvasToDisplaySize(gl.canvas);
        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
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
        const vao = vertexArrayInfo.vao;
        gl.bindVertexArray(vao);
            gl.bufferSubData(gl.ARRAY_BUFFER, 0, cellPositions);
        gl.bindVertexArray(null);

        twgl.drawBufferInfo(gl, vertexArrayInfo, gl.TRIANGLES, vertexArrayInfo.numelements, 0, size);
    }
}