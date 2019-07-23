import { Universe, Force, Config } from "spherro";

import Renderer from "./renderer";
import FPSCounter from "./fpscounter"

const WIDTH = 700;
const HEIGHT = 700;

const config = Config.new(0.4, 0.8, 50, 10);
var universe = Universe.new(WIDTH, HEIGHT, config);

const canvas = document.getElementById('spherro-canvas');
const fpsCounter = new FPSCounter(20);
const renderer = new Renderer(canvas, WIDTH, HEIGHT, universe.get_size());

var shouldReset = false;
var mouseX = 0;
var mouseY = 0;
var isMousedown = false;
var isStable = true;
var nFrames = 0;

const renderLoop = (currentTime) => {
    fpsCounter.register(currentTime);
    renderer.draw(universe, currentTime);

    for(var i=0; i<2; i++) {
        universe.update(0.005);
        if(universe.is_unstable()) {
            isStable = false;
            document.getElementById('stability').innerText = 'Unstable';
            document.getElementById('stability').style.color = 'red';
        }
    }

    if(shouldReset) {
        universe = Universe.new(WIDTH, HEIGHT, config);
        isStable = true;
        document.getElementById('stability').innerText = 'Stable';
        document.getElementById('stability').style.color = 'green';
        shouldReset = false;
    }

    universe.clear_forces();
    if(isMousedown) {
        const force = Force.new(mouseX*WIDTH, mouseY*HEIGHT, 2e8, 100.0);
        universe.add_force(force);
    }

    if(nFrames % 20 == 0) {
        const fps = fpsCounter.smoothFPS();
        document.getElementById('fps').innerText = fps.toFixed(1) + ' FPS';
    }

    nFrames += 1;
    requestAnimationFrame(renderLoop);
};
requestAnimationFrame(renderLoop);

document.addEventListener('keypress', function(e) {
    if(e.key === 'r') {
        shouldReset = true;
    } else if (e.key === 'p') {
        universe.queue_spawn_particles(5, 25.0, HEIGHT - 25.0);
    } else if (e.key === 'o') {
        universe.queue_despawn_particles(5);
    }
})

function getCursorPosition(canvas, event) {
    const rect = canvas.getBoundingClientRect();
    const x = (event.clientX - rect.left) / rect.width;
    const y = (rect.height - (event.clientY - rect.top)) / rect.height;
    return {x, y};
}

canvas.addEventListener('mousemove', function(e) {
    const pos = getCursorPosition(canvas, e);
    mouseX = pos.x;
    mouseY = pos.y;
})

document.addEventListener('mousedown', function(e) {
    isMousedown = true;
    const pos = getCursorPosition(canvas, e);
    mouseX = pos.x;
    mouseY = pos.y;
})

document.addEventListener('mouseup', function(e) {
    isMousedown = false;
})

canvas.addEventListener('touchmove', function(e) {
    e.preventDefault();
    const pos = getCursorPosition(canvas, e.changedTouches[0]);
    mouseX = pos.x;
    mouseY = pos.y;
})

document.addEventListener('touchstart', function(e) {
    e.preventDefault();
    const pos = getCursorPosition(canvas, e.changedTouches[0]);
    mouseX = pos.x;
    mouseY = pos.y;
    isMousedown = true;
})

document.addEventListener('touchend', function(e) {
    e.preventDefault();
    isMousedown = false;
})