# spherro

A smoothed particle hydrodynamics fluid simulator. Built with rust, and compiled to wasm.

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

## License

[MIT License](LICENSE)