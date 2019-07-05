import { Universe, Strategy } from "spherro";
import { Blob as SpherroBlob } from "spherro";
import { memory } from "spherro/spherro_bg";
import * as twgl from "twgl.js";

const WIDTH = 700;
const HEIGHT = 700;
const VS_FILE = 'shaders/vs.glsl';
const FS_FILE = 'shaders/fs_retro.glsl';

const strategy = Strategy.DAMBREAK;
const universe = Universe.new(WIDTH, HEIGHT, strategy);
const spherroBlob = SpherroBlob.new(50, 50);

const canvas = document.getElementById('spherro-canvas');
canvas.width = WIDTH;
canvas.height = HEIGHT;
const gl = canvas.getContext('webgl');

// This is a mess
fetch(VS_FILE)
    .then(response => response.text())
    .then(vs => {
        fetch(FS_FILE)
            .then(response => response.text())
            .then(fs => {
                load(vs, fs);
            });
    });

// Another mess.
function load(vs, fs) {
    const programInfo = twgl.createProgramInfo(gl, [vs, fs]);
    const arrays = {
        position: [-1, -1, 0, 1, -1, 0, -1, 1, 0, -1, 1, 0, 1, -1, 0, 1, 1, 0],
    };
    const bufferInfo = twgl.createBufferInfoFromArrays(gl, arrays);

    const texture = twgl.createTexture(gl, {
        mag: gl.NEAREST,
        min: gl.LINEAR,
        format: gl.LUMINANCE,
        src: new Uint8Array(50*50),
        width: 50,
        height: 50,
        wrap: gl.CLAMP_TO_EDGE,
    });

    var lastTime = 0.0;
    const renderLoop = (currentTime) => {
        const dt = currentTime - lastTime;
        lastTime = currentTime;

        for(var i=0; i<=10; i++) {
            universe.update(0.001);
        }

        spherroBlob.set_from_universe(universe);
        const blobPtr = spherroBlob.get_data();
        const blobArr = new Uint8Array(memory.buffer, blobPtr, 50*50);
        const arr = new Uint8Array(50*50);
        for(var i=0; i<50*50; i++) { arr[i] = blobArr[i]; }
        twgl.setTextureFromArray(gl, texture, arr, {
            wrap: gl.CLAMP_TO_EDGE,
            mag: gl.NEAREST,
            min: gl.LINEAR,
            format: gl.LUMINANCE,
            width: 50,
            height: 50
        });

        twgl.resizeCanvasToDisplaySize(gl.canvas);
        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);

        const uniforms = {
            time: currentTime * 0.001,
            resolution: [gl.canvas.width, gl.canvas.height],
            tex: texture,
        };

        gl.useProgram(programInfo.program);
        twgl.setBuffersAndAttributes(gl, programInfo, bufferInfo);
        twgl.setUniforms(programInfo, uniforms);
        twgl.drawBufferInfo(gl, bufferInfo);

        requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
}
