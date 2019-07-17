import { Universe, Strategy, Force } from "spherro";

import Renderer from "./renderer";
import FPSCounter from "./fpscounter"

const WIDTH = 700;
const HEIGHT = 700;

const strategy = Strategy.DAMBREAK;
var universe = Universe.new(WIDTH, HEIGHT, strategy);
const canvas = document.getElementById('spherro-canvas');
const fpsCounter = new FPSCounter(10);
const renderer = new Renderer(canvas, WIDTH, HEIGHT, universe.get_size());

var shouldReset = false;
var mouseX = 0;
var mouseY = 0;
var isMousedown = false;
var forcePower = 1.0;

const renderLoop = (currentTime) => {
    fpsCounter.register(currentTime);
    renderer.draw(universe, currentTime);

    for(var i=0; i<2; i++) {
        universe.update(0.005);
    }

    if(shouldReset) {
        universe = Universe.new(WIDTH, HEIGHT, strategy);
        shouldReset = false;
    }

    universe.clear_forces();
    if(isMousedown) {
        forcePower = Math.min(forcePower * 10.0, 2e8);
        const force = Force.new(mouseX, mouseY, forcePower, 100.0);
        universe.add_force(force);
    } else {
        forcePower = 1.0;
    }

    const fps = fpsCounter.smoothFPS();
    document.getElementById('fps').innerText = fps.toFixed(2) + ' FPS';

    requestAnimationFrame(renderLoop);
};
requestAnimationFrame(renderLoop);

document.addEventListener('keypress', function(e) {
    if(e.key == 'r') {
        shouldReset = true;
    }
})

function getCursorPosition(canvas, event) {
    const rect = canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = rect.height - (event.clientY - rect.top);
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
    const pos = getCursorPosition(canvas, e.changedTouches[0]);
    mouseX = pos.x;
    mouseY = pos.y;
})

document.addEventListener('touchstart', function(e) {
    const pos = getCursorPosition(canvas, e.changedTouches[0]);
    mouseX = pos.x;
    mouseY = pos.y;
    isMousedown = true;
})

document.addEventListener('touchend', function(e) {
    isMousedown = false;
})