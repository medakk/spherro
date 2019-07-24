# spherro
[![Crates.io](https://img.shields.io/crates/v/spherro.svg)](https://crates.io/crates/spherro)

A smoothed particle hydrodynamics fluid simulator. Built with rust, and compiled to wasm. Check out the demo [here](https://apps.karthikkaranth.me/spherro/).

<p align="center">
   <a href="https://giant.gfycat.com/JovialBabyishBalloonfish.webm"><img src="https://thumbs.gfycat.com/JovialBabyishBalloonfish-small.gif" alt="gif"></a>
</p>

## Building

### Dependencies
All instructions have been tested on Ubuntu 18.04, with the versions:
* rust 1.35.0
* wasm-pack 0.8.1
* npm 6.9.0
* node v10.16.0

### Steps to build
* Install:
    * [The standard rust toolchain](https://www.rust-lang.org/tools/install)
    * [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
    * [npm](https://www.npmjs.com/get-npm)
* Clone this repo: `git clone https://github.com/medakk/spherro`
* In the root folder of the repo, run `wasm-pack build`
* In the `www` folder, run: 

```
npm install
npm run start
```

This will start a server(defaults to http://localhost:8080) serving spherro.

## Debugging

Running `cargo run --bin spherro-bin --release` starts a [kiss3d](https://docs.rs/kiss3d/0.20.1/kiss3d/)-based viewer that can be used to debug the simulator without going through the browser.

Running `cargo bench` starts a headless dambreak simulation with a fixed time step. This can be used to test performance changes.

## References

* [SPH Fluids in Computer Graphics](https://cg.informatik.uni-freiburg.de/publications/2014_EG_SPH_STAR.pdf), _EUROGRAPHICS 2014_

## License

[MIT License](LICENSE)
