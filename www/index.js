import { Universe, Strategy } from "spherro";
import { memory } from "spherro/spherro_bg";

const WIDTH = 700;
const HEIGHT = 700;

const strategy = Strategy.DAMBREAK
const universe = Universe.new(WIDTH, HEIGHT, strategy);
const size = universe.get_size();

const canvas = document.getElementById('spherro-canvas');
canvas.width = WIDTH;
canvas.height = HEIGHT;

const ctx = canvas.getContext('2d');

var lastTime = 0.0;

const renderLoop = (currentTime) => {
    const dt = currentTime - lastTime;
    lastTime = currentTime;

    const cellsPtr = universe.get_data();
    const stride = universe.get_data_stride() / 4; // 4 bytes a float. TODO: needs more thought
    const cells = new Float32Array(memory.buffer, cellsPtr, size * stride);

    ctx.clearRect(0, 0, WIDTH, HEIGHT);
    for(var i=0; i<size; i++) {
        const x = cells[i*stride+0];
        const y = HEIGHT - cells[i*stride+1];
        const z = cells[i*stride+2];

        ctx.beginPath();
        ctx.arc(x, y, 10, 0, 2*Math.PI);
        ctx.fill();
    }

    for(var i=0; i<=10; i++) {
        universe.update(0.001);
    }

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);