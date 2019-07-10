import { Universe, Strategy } from "spherro";

import Renderer from "./renderer";
import FPSCounter from "./fpscounter"

export function main() {
    const WIDTH = 700;
    const HEIGHT = 700;

    const strategy = Strategy.DAMBREAK
    var universe = Universe.new(WIDTH, HEIGHT, strategy);

    const canvas = document.getElementById('spherro-canvas');
    canvas.width = WIDTH;
    canvas.height = HEIGHT;

    const renderer = new Renderer(canvas, WIDTH, HEIGHT);
    const fpsCounter = new FPSCounter(10);

    var shouldReset = false;
    const renderLoop = (currentTime) => {
        fpsCounter.register(currentTime);
        renderer.draw(universe);

        for(var i=0; i<5; i++) {
            universe.update(0.002);
        }

        if(shouldReset) {
            universe = Universe.new(WIDTH, HEIGHT, strategy);
            shouldReset = false;
        }

        const fps = fpsCounter.smoothFPS();
        renderer.draw_fps(fps);

        requestAnimationFrame(renderLoop);
    };
    requestAnimationFrame(renderLoop);

    document.addEventListener('keypress', function(e) {
        if(e.key == 'r') {
            shouldReset = true;
        }
    })

}