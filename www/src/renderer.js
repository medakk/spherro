import { memory } from "spherro/spherro_bg";

export default class Renderer {
    constructor(canvas, width, height) {
        this.ctx = canvas.getContext('2d');
        this.width = width;
        this.height = height;
    }

    draw(universe) {
        const ctx = this.ctx;
        const cellsPtr = universe.get_data();
        const size = universe.get_size();
        const stride = universe.get_data_stride() / 4; // 4 bytes a float. TODO: needs more thought
        const cells = new Float32Array(memory.buffer, cellsPtr, size * stride);

        ctx.fillStyle = '#00';
        ctx.clearRect(0, 0, this.width, this.height);
        ctx.globalCompositeOperation = 'lighter';

        for(var i=0; i<size; i++) {
            const x = cells[i*stride+0];
            const y = this.height - cells[i*stride+1];

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
    }

    draw_fps(fps) {
        const ctx = this.ctx;

        ctx.fillStyle = 'red';
        ctx.font = '20px Arial';
        ctx.fillText(fps.toFixed(1) + ' FPS', 20, 20);
    }
}