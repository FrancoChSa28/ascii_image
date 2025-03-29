# Rust ASCII Image Generator

A simple Rust-based tool that converts images into ASCII art, supporting grayscale ANSI output for the terminal.

## Features
- Convert images to ASCII using brightness levels.
- Resizes images while maintaining aspect ratio.
- Works with common image formats (PNG, JPG, BMP, etc.).

## Installation

### Prerequisites
- Ensure you have **Rust** installed. If not, install it via [Rustup](https://rustup.rs/):
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Install required dependencies:
  ```sh
  cargo install image
  ```

### Clone the Repository
```sh
 git clone https://github.com/FrancoChSa28/ascii_image.git
 cd ascii_image
```

### Build the Project
```sh
cargo build --release
```

## Usage
### Convert an Image to ASCII
Run the program with an image file as input:
```sh
cargo run --release -- --imput path/to/image.png
```
This will generate and display the ASCII art in a explorer windows. Also it will be saved in a file **ascii_art.png**.

## Configuration Options
| Option     | Description |
|------------|-------------|
| `--input <FILE>` | Path to image |
| `--width <N>` | Sets the output width (default: 100) |
| `--contrast <N>` | Sets the output contrast (default: 10) |
| `--color` | Sets the output color (default: false (grayscale) ) |

## License
This project is licensed under the MIT License.

## Contributions
Pull requests and improvements are welcome! Feel free to open an issue for feature requests or bug reports.

## Author
**Your Name**  
GitHub: [FrancoChSa28](https://github.com/FrancoChSa28)

