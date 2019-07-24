import Vue from 'vue';
import VueSlider from 'vue-slider-component'
import 'vue-slider-component/theme/material.css'

import { Universe, Force, Config } from "spherro";
import Renderer from "./renderer";
import FPSCounter from "./fpscounter"

// Register Vue component
Vue.component('VueSlider', VueSlider)

// Disable the loading overlay
document.querySelector('.loading').remove();

const WIDTH = 700;
const HEIGHT = 700;

const config = Config.new(0.4, 0.8, 50, 10);
var universe = Universe.new(WIDTH, HEIGHT, config);

const canvas = document.getElementById('spherro-canvas');
const fpsCounter = new FPSCounter(20);
const renderer = new Renderer(canvas, WIDTH, HEIGHT, universe.get_size());

const app = new Vue({
    el: '.controls',
    data: {
        fps: 60.0,
        isStable: true,
        desiredParticleCount: 500,
        particleCount: 500,
        isVueLoaded: true,
    },
});

var shouldReset = false;
var mouseX = 0;
var mouseY = 0;
var isMousedown = false;
var nFrames = 0;

const renderLoop = (currentTime) => {
    fpsCounter.register(currentTime);
    renderer.draw(universe, currentTime);

    for(var i=0; i<2; i++) {
        universe.update(0.005);
        if(universe.is_unstable()) {
            app.isStable = false;
        }
    }

    if(shouldReset) {
        universe = Universe.new(WIDTH, HEIGHT, config);
        app.isStable = true;
        shouldReset = false;
    }

    app.particleCount = universe.get_size();

    // Update forces
    universe.clear_forces();
    if(isMousedown) {
        const force = Force.new(mouseX*WIDTH, mouseY*HEIGHT, 2e8, 100.0);
        universe.add_force(force);
    }

    // Add events
    console.assert(app.desiredParticleCount % 5 === 0);
    const logicalSize = universe.get_size() + universe.get_queue_diff();
    if(app.desiredParticleCount > logicalSize) {
        universe.queue_spawn_particles(5, 25.0, HEIGHT - 25.0);
    } else if(app.desiredParticleCount < logicalSize) {
        universe.queue_despawn_particles(2);
    }

    if(nFrames % 20 === 0) {
        const fps = fpsCounter.smoothFPS();
        app.fps = fps.toFixed(1) + ' FPS';
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