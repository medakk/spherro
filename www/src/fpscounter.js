export default class FPSCounter {
    constructor(sampleCount) {
        this.sampleCount = sampleCount;
        this.timestamps = [];
        this.currIdx = 0;
        this.lastTime = 0.0;
    }

    register(currTime) {
        const dt = (currTime - this.lastTime) / 1000.0;
        this.lastTime = currTime

        this.currIdx = (this.currIdx + 1) % this.sampleCount;
        if(this.timestamps.length < this.sampleCount) {
            this.timestamps.push(dt);
        } else {
            this.timestamps[this.currIdx] = dt;
        }
    }

    FPS() {
        return 1.0 / this.timestamps[this.currIdx];
    }

    dt() {
        return this.timestamps[this.currIdx];
    }

    smoothFPS() {
        return this.timestamps.length / this.timestamps.reduce((dt1, dt2) => (dt1 + dt2));
    }
}