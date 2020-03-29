# rusty-life

Rust implementation of Conway's Game of Life. Casually written to pass the time during a weekend of coronavirus self-isolation.

This implementation simulates the Game of Life and renders it on the terminal. The size of the simulated grid is automatically set to the character width/height of your terminal.

## Running the Game

To run the game, simply clone this crate and type:

```
cargo run
```

Or after installing the crate using `cargo install rusty-life`, run:

```
rusty-life
```

## Options

```
Usage: rusty-life [options]

Options:
    --fps=<fps>  Frames/grid updates to render/run per second [default: 5].
```
