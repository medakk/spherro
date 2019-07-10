import { Universe, Strategy } from "spherro";

import Renderer from "./renderer";
import FPSCounter from "./fpscounter"

export function main() {
    const WIDTH = 700;
    const HEIGHT = 700;

    const strategy = Strategy.DAMBREAK;
    var universe = Universe.new(WIDTH, HEIGHT, strategy);
    const canvas = document.getElementById('spherro-canvas');
    const fpsCounter = new FPSCounter(10);
    const renderer = new Renderer(canvas, WIDTH, HEIGHT);

    var shouldReset = false;

    const renderLoop = (currentTime) => {
        fpsCounter.register(currentTime);
        renderer.draw(universe, currentTime);

        for(var i=0; i<5; i++) {
            universe.update(0.002);
        }

        if(shouldReset) {
            universe = Universe.new(WIDTH, HEIGHT, strategy);
            shouldReset = false;
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

}