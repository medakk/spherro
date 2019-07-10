import { Universe, Strategy } from "spherro";
import { memory } from "spherro/spherro_bg";

export function main() {
    const WIDTH = 700;
    const HEIGHT = 700;

    const strategy = Strategy.DAMBREAK
    var universe = Universe.new(WIDTH, HEIGHT, strategy);
    const size = universe.get_size();

    const canvas = document.getElementById('spherro-canvas');
    canvas.width = WIDTH;
    canvas.height = HEIGHT;

    const ctx = canvas.getContext('2d');

    var lastTime = 0.0;
    var shouldReset = false;
    var dts = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    var dtIdx = 0;

    const renderLoop = (currentTime) => {
        const dt = (currentTime - lastTime) / 1000.0;
        dts[dtIdx] = dt; dtIdx = (dtIdx + 1) % 7;
        lastTime = currentTime;

        const cellsPtr = universe.get_data();
        const stride = universe.get_data_stride() / 4; // 4 bytes a float. TODO: needs more thought
        const cells = new Float32Array(memory.buffer, cellsPtr, size * stride);

        ctx.fillStyle = '#00';
        ctx.clearRect(0, 0, WIDTH, HEIGHT);
        ctx.globalCompositeOperation = 'lighter';

        for(var i=0; i<size; i++) {
            const x = cells[i*stride+0];
            const y = HEIGHT - cells[i*stride+1];

            const r1 = 10;
            const r2 = 20;
            var grd = ctx.createRadialGradient(x, y, r1, x, y, r2);
            grd.addColorStop(0.0, '#0023edff');
            grd.addColorStop(0.2, '#0023eddd');
            grd.addColorStop(1.0, '#0023ed22');
            ctx.fillStyle = grd;

            ctx.beginPath();
            ctx.arc(x, y, r2, 0, 2*Math.PI);
            ctx.fill();
        }

        for(var i=0; i<5; i++) {
            universe.update(0.002);
        }

        if(shouldReset) {
            universe = Universe.new(WIDTH, HEIGHT, strategy);
            shouldReset = false;
        }

        const fps = 1.0 / (dts.reduce((a, b) => a+b)/7.0);
        ctx.fillStyle = 'red';
        ctx.font = '20px Arial';
        ctx.fillText(fps.toFixed(1) + ' FPS', 20, 20);

        requestAnimationFrame(renderLoop);
    };
    requestAnimationFrame(renderLoop);

    document.addEventListener('keypress', function(e) {
        if(e.key == 'r') {
            shouldReset = true;
        }
    })

}