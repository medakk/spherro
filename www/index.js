import { Universe, Strategy } from "spherro";
import { Blob as SpherroBlob } from "spherro";
import { memory } from "spherro/spherro_bg";

const WIDTH = 700;
const HEIGHT = 700;

const strategy = Strategy.DAMBREAK
const universe = Universe.new(WIDTH, HEIGHT, strategy);
const spherroBlob = SpherroBlob.new(50, 50);
const size = universe.get_size();

const canvas = document.getElementById('spherro-canvas');
canvas.width = WIDTH;
canvas.height = HEIGHT;
const ctx = canvas.getContext('2d');

var lastTime = 0.0;

const renderLoop = (currentTime) => {
    const dt = currentTime - lastTime;
    lastTime = currentTime;

    ctx.fillStyle = '#000000';
    ctx.fillRect(0, 0, WIDTH, HEIGHT);
    for(var i=0; i<=10; i++) {
        universe.update(0.001);
    }

    spherroBlob.set_from_universe(universe);
    const blobPtr = spherroBlob.get_data();
    const blobArr = new Uint8ClampedArray(memory.buffer, blobPtr, 50*50);
    for(var y=0; y<50; y++) {
        for(var x=0; x<50; x++) {
            var c = blobArr[y*50+x];
            ctx.fillStyle = 'rgb(' + Math.floor(c) + ',' +
                                    Math.floor(c) + ',' +
                                    Math.floor(c) + ')';

            ctx.beginPath();
            const y_inv = 50 - y;
            ctx.fillRect(x*14, y_inv*14, 10, 10);
        }
    }

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);