# Rust Game Window

This project is a simple Rust implementation of a game window using the `ggez` library. It creates a window with a movable white square that can be controlled using arrow keys.

## Prerequisites

- Rust programming language
- Cargo package manager

## Setup

1. Clone this repository:
   ```
   git clone ${TODO}
   cd rust_game
   ```

2. Ensure you have the latest stable version of Rust:
   ```
   rustup update stable
   ```

## Running the Game

To run the game, use the following command in the project directory:

```
cargo run
```

### Running on WSL (Windows Subsystem for Linux)

If you're using WSL, you'll need to set up an X server on Windows and use the following command to run the game:

```
DISPLAY=:0 cargo run
```

You may also need to set the `LIBGL_ALWAYS_INDIRECT` environment variable:

```
export LIBGL_ALWAYS_INDIRECT=1
```

## Game Controls

- Use the arrow keys to move the white square:
  - Left Arrow: Move left
  - Right Arrow: Move right
  - Up Arrow: Move up
  - Down Arrow: Move down

## Project Structure

- `Cargo.toml`: Contains the project configuration and dependencies.
- `src/main.rs`: Contains the main game logic, including window creation, event handling, and rendering.

## Customization

You can customize the game by modifying the constants at the top of `main.rs`:

- `WINDOW_WIDTH` and `WINDOW_HEIGHT`: Change the size of the game window.
- `SQUARE_SIZE`: Adjust the size of the movable square.
- `SQUARE_SPEED`: Modify the speed at which the square moves.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).
