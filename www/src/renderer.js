import * as twgl from "twgl.js";

import { memory } from "spherro/spherro_bg";

import VERTEX_SHADER from './shaders/particle_vs.glsl';
import FRAGMENT_SHADER from './shaders/particle_fs.glsl';

const FS_FILE = 'shaders/particle_fs.glsl';

export default class Renderer {
    constructor(canvas, width, height) {
        this.width = canvas.width = width;
        this.height = canvas.height = height;
        this.gl = canvas.getContext('webgl');

        this.init(this.gl);
    }

    init(gl) {
        const programInfo = twgl.createProgramInfo(gl, [VERTEX_SHADER, FRAGMENT_SHADER]);
        const arrays = {
            position: [-1, -1, 0, 1, -1, 0, -1, 1, 0, -1, 1, 0, 1, -1, 0, 1, 1, 0],
        };
        const bufferInfo = twgl.createBufferInfoFromArrays(gl, arrays);

        this.glInfo = {
            programInfo: programInfo,
            bufferInfo: bufferInfo,
        };
    }

    draw(universe, currentTime) {
        const gl = this.gl;
        const cellsPtr = universe.get_data();
        const size = universe.get_size();
        const stride = universe.get_data_stride() / 4; // 4 bytes a float. TODO: needs more thought
        const cells = new Float32Array(memory.buffer, cellsPtr, size * stride);

        const {programInfo, bufferInfo} = this.glInfo;

        twgl.resizeCanvasToDisplaySize(gl.canvas);
        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
        const uniforms = {
            time: currentTime * 0.001,
            resolution: [gl.canvas.width, gl.canvas.height],
        };
        gl.useProgram(programInfo.program);
        twgl.setBuffersAndAttributes(gl, programInfo, bufferInfo);
        twgl.setUniforms(programInfo, uniforms);
        twgl.drawBufferInfo(gl, bufferInfo);
    }
}