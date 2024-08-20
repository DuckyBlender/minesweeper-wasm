# Minesweeper Game in Rust with WebAssembly

This project is a simple implementation of the classic Minesweeper game in Rust, compiled to WebAssembly using wasm-pack.

## About the Game

The game is a standard Minesweeper game with a grid of cells, some of which contain mines. The player can reveal cells to try to clear the grid without hitting a mine.

## Features

* Randomly generated mine locations
* Reveal cells to clear the grid
* Flag cells to mark potential mines
* Reset the game to start over

## Running the Game

To run the game, follow these steps:

1. Install wasm-pack by running the following command:
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
2. Clone this repository and navigate to the project directory.
3. Run the following command to build the game:
```bash
wasm-pack build --target web
```
4. Open the `index.html` file in your web browser to play the game.

## License

This project is licensed under the GPL-3.0 License.

## Acknowledgments

This project uses the wasm-pack tool to compile the Rust code to WebAssembly. It also uses the XORShift algorithm to generate pseudorandom numbers.