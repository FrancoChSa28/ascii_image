use std::fs;
use ascii_image::ascii::{ascii_black_and_white, ascii_color};
use clap::{self, command, Parser};

#[derive(Parser, Debug)]
#[command(name="ASCII Image Generator", author, version = "1.0", about = "Converts images to ASCII art", long_about = None)]
struct Args {
    /// Path to the input image file
    #[arg(short, long)]
    input: String,

    /// Output width of the ASCII image (default: 80)
    #[arg(short, long, default_value_t = 100)]
    width: u32,

    /// Enable ANSI contrast adjustment (default: 10)
    #[arg(short, long, default_value_t = 10)]
    contrast: u32,

    /// Enable ANSI color output (default: false)
    #[arg(long, default_value_t = false)]
    color: bool,

    /// Open the generated ASCII art image (default: false)
    #[arg(long, default_value_t = false)]
    open: bool,
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");

    let args = Args::parse();
    let (width, contrast) = (args.width as i32, args.contrast as i32);
    let img_name = args.input.as_str();
    let color = args.color;
    let open = args.open;

    // Check inputs
    fs::metadata(img_name).expect("File not found");
    let width_range = 10..=1000;
    assert!(width_range.contains(&width), "Width should be between 10 and 1000");
    let contrast_range = -10..=10;
    assert!(contrast_range.contains(&contrast), "Contrast should be between -10 and 10");

    // ASCII characters for mapping pixel brightness.
    let mut density = String::from("$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'.            ");
    density = density.chars().rev().collect::<String>();
    // print!("Length: {}", density.len());
    let mut n = density.len() as i32;
    density.truncate((n - 11 + contrast) as usize);
    n = density.len() as i32;
    // print!("Length: {}", density.len());


    match color {
        false => {
            // Convert the image to black and white ASCII art.
            ascii_black_and_white(img_name, width, &density, n);
        },
        true => {
            // Convert the image to color ASCII art.
            ascii_color(img_name, width, &density, n);
        }
    }

    if open {
        // Open the generated ASCII art image.
        open::that("ascii_art.png").expect("Error opening image 'ascii_art.png'");
    }
}