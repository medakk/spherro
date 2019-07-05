import { Universe, Strategy } from "spherro";
import { Blob as SpherroBlob } from "spherro";
import { memory } from "spherro/spherro_bg";

const WIDTH = 700;
const HEIGHT = 700;

const strategy = Strategy.DAMBREAK
const universe = Universe.new(WIDTH, HEIGHT, strategy);
const spherroBlob = SpherroBlob.new(10, 10);
const size = universe.get_size();

const canvas = document.getElementById('spherro-canvas');
canvas.width = WIDTH;
canvas.height = HEIGHT;
const ctx = canvas.getContext('2d');

var lastTime = 0.0;

const renderLoop = (currentTime) => {
    const dt = currentTime - lastTime;
    lastTime = currentTime;

    for(var i=0; i<=10; i++) {
        universe.update(0.001);
    }

    spherroBlob.set_from_universe(universe);
    const blobPtr = spherroBlob.get_data();
    const blobArr = new Uint8ClampedArray(memory.buffer, blobPtr, 10*10);
    for(var y=0; y<10; y++) {
        for(var x=0; x<10; x++) {
            var c = blobArr[y*10+x];
            ctx.fillStyle = 'rgb(' + Math.floor(c) + ',' +
                                     Math.floor(0) + ',' +
                                     Math.floor(0) + ')';
            ctx.beginPath();
            ctx.arc(100+x*5, 100+y*5, 2, 0, 2*Math.PI);
            ctx.fill();
        }
    }

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);