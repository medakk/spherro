import { Universe } from "spherro";
import { memory } from "spherro/spherro_bg";

const universe = Universe.new();
const size = universe.get_size();

const canvas = document.getElementById('spherro-canvas');
const width = 700;
const height = 700;
canvas.width = width;
canvas.height = height;

const ctx = canvas.getContext('2d');

var lastTime = 0.0;

const renderLoop = (currentTime) => {
    const dt = currentTime - lastTime;
    lastTime = currentTime;

    const cellsPtr = universe.get_particle_positions();
    const cells = new Float32Array(memory.buffer, cellsPtr, size * 3);

    ctx.clearRect(0, 0, width, height);
    for(var i=0; i<size; i++) {
        const x = cells[i*3+0];
        const y = cells[i*3+1];
        const z = cells[i*3+2];

        ctx.beginPath();
        ctx.arc(x, y, 10, 0, 2*Math.PI);
        ctx.fill();
    }

    universe.update(dt / 1000.0);

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);