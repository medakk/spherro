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

    ctx.fillStyle = '#00';
    ctx.clearRect(0, 0, WIDTH, HEIGHT);
    for(var i=0; i<size; i++) {
        const x = cells[i*stride+0];
        const y = HEIGHT - cells[i*stride+1];

        const r1 = 10;
        const r2 = 20;
        var grd = ctx.createRadialGradient(x, y, r1, x, y, r2);
        grd.addColorStop(0.0, '#0023edff');
        grd.addColorStop(0.2, '#0023eddd');
        grd.addColorStop(1.0, '#0023ed22');
        ctx.globalCompositeOperation = 'lighter';

        ctx.fillStyle = grd;
        ctx.beginPath();
        ctx.arc(x, y, r2, 0, 2*Math.PI);
        ctx.fill();
    }

    for(var i=0; i<=10; i++) {
        universe.update(0.001);
    }

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);