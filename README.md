# RPG Tournament
My attempt at creating an action RPG in [Rust](https://www.rust-lang.org/) using the [Bevy](https://bevyengine.org/) game engine.

[recording-1.webm](https://github.com/user-attachments/assets/c8cf1597-cafd-430f-929e-07cd0c77e645)

## Build / Run Instructions
Make sure you have an up-to-date version of [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Build and Run
```
cargo run
```

### Build
```
cargo build
```

## Debug Mode
Press CTRL+D to toggle debug mode on and off.

### Controls
- **P**: Toggle camera projection between orthographic and perspective.
    - Useful for debugging issues with map geometry.
- **F**: Toggle flycam.
    - Camera will detach from the player.
    - Use WASD keys to move on the X and Z axes.
    - Use Space + Shift to move up and down the Y axis.
- **F5**: Reload all maps in the current area.
    - Useful, as maps can update while the game is running, avoiding the need to restart it.
