# Combine Derby

A disappointingly literal take on COMBINE for the [Bevy game jam](https://itch.io/jam/bevy-jam-2).

This project has been reincarnated as a [Godot project](https://github.com/RichTeaMan/combine-derby).

## Running

Combine Derby can be run locally using Cargo:

```bash
cargo run
```

## Controls

Move the Combine with WASD.

* F3 to togglee debug info.
* F4 to switch camera.
* F6 to toggle sound muting.

### Building WASM

[Web Assembly](https://webassembly.org/) allows this program to be run in a browser.

Combine Derby builds WASM files to the docs directory so it can be hosted in Github Pages.

```bash
rustup target install wasm32-unknown-unknown && cargo install wasm-bindgen-cli && \
cargo build --all-features --target wasm32-unknown-unknown && \
wasm-bindgen --out-dir ./docs/ --target web target/wasm32-unknown-unknown/debug/combine-derby.wasm && \
cp assets/ docs/. -r
```

The WASM build can also be tested locally, **run this in the docs folder**:

```bash
python3 -m http.server
```

Note that this doesn't work so well in Firefox due to HTTPS constraints. Chrome has a better time of it.

## Asset credits

Soil texture: https://www.deviantart.com/fabooguy/art/Dirt-Ground-Texture-Tileable-2048x2048-441212191.

Side hay bale texture: https://create.vista.com/unlimited/stock-photos/220985178/stock-photo-hay-bale-isolated-white-background/

Moo sound: https://www.freesoundslibrary.com/cow-moo-sound/

Background music: https://www.bensound.com/royalty-free-music/track/jazzy-frenchy-upbeat-funny

Engine sound sample: https://opengameart.org/content/engine-loop-heavy-vehicletank

Hay sound sample: https://www.youtube.com/watch?v=lMlY5yFZ-b8

Smoke particle: https://www.pngkit.com/downpic/u2q8q8u2o0t4q8t4_fog-png-transparent-images-smoke-particle-texture-png/
