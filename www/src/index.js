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
var isStable = true;

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
        universe = Universe.new(WIDTH, HEIGHT, strategy);
        isStable = true;
        document.getElementById('stability').innerText = 'Stable';
        document.getElementById('stability').style.color = 'green';
        shouldReset = false;
    }

    universe.clear_forces();
    if(isMousedown) {
        const force = Force.new(mouseX, mouseY, 2e8, 100.0);
        universe.add_force(force);
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

// TODO: This code doesn't work on resizing the window. assumes 1:1 correspondence with
// simulation or something
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