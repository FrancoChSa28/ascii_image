use std::fs;
use image::{imageops, open, Rgb, RgbImage};
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

    /// Enable ANSI color output
    #[arg(short, long, default_value_t = 10)]
    contrast: u32,
}


fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    // let args: Vec<String> = env::args().collect();
    // let img_name =  args.get(1).expect("File name not provided");
    // let width: u32 = args.get(2).map_or(100, |s| s.parse().unwrap());
    // let contrast: i32 = args.get(3).map_or(10, |s| s.parse().unwrap());

    let args = Args::parse();
    let (width, contrast) = (args.width as i32, args.contrast as i32);
    let img_name = args.input.as_str();

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

    // Read the image and convert it to grayscale.
    let img = open(img_name).unwrap().to_luma8();
    let (orig_width, orig_height) = img.dimensions();
    let r = orig_height as f64 / orig_width as f64;
    let height = (width as f64 * r * 0.5) as u32;
    let img = imageops::resize(&img, width as u32, height, image::imageops::FilterType::Lanczos3);

    // Print some information about the image.
    // println!("Image: {}x{} -> {}x{}", orig_width, orig_height, width, height);

    // Save the grayscale image.
    img.save("grayscale.png").expect("Error saving grayscale image");
    
    // Create a new RGB image to store the ASCII art.
    let mut ascii_img = RgbImage::new(width as u32 * 8, height * 16);
    let font = include_bytes!("../simple-8x16.font");
    let font_width = 8;
    let font_height = 16;

    // Map pixel brightness to ASCII characters.
    for y in 0..height {
        for x in 0..width {
            let p = img.get_pixel(x as u32, y).0[0] as i32;
            let k = (p as f64 / 256.0 * n as f64).floor() as usize;

            let character = density.chars().nth((n - 1 - (k as i32) ) as usize).unwrap_or_else(|| ' ');

            // Print p, k, character
            // println!("y: {} \tx: {} \tp: {} \tk: {} \tchar: {} \tn: {}", y, x, p, k, character, n);
            
            // print!("{}", character);

            let x_offset = x * 8; // Each character occupies 8 pixels horizontally
            let y_offset = y * 16; // Each character occupies 16 pixels vertically
            draw_character(&mut ascii_img, character, font, font_width, font_height, x_offset as u32, y_offset);

            // draw_character(&mut ascii_img, character, x as u32, y);

            // match density.chars().nth((n - 1 - (k as i32) ) as usize) {
            //     Some(character) => print!("{}", character), 
            //     None => print!(" "),
            // }

            // print!("{}", density.chars().nth((n - 1 - (k as i32) ) as usize).unwrap());
        }
        // println!();
    }

    // Save the ASCII art image.

    match ascii_img.save("ascii_art.png") {
        Ok(_) => {
            println!("Image saved successfully");
            match open::that("ascii_art.png") {
                Ok(_) => (),
                Err(e) => println!("Error opening image: {}", e),
            }
        },
        Err(e) => println!("Error saving image: {}", e),
    }
}


// Function to draw a character onto the image
fn draw_character(image: &mut RgbImage, character: char, font: &[u8], font_width: u32, font_height: u32, x_offset: u32, y_offset: u32) {
    // Map the ASCII character to its corresponding position in the font
    let index = (character as usize) * (font_height as usize);
    // println!("Index: {}", index);
    // Draw the character onto the image
    for y in 0..font_height {
        for x in 0..font_width {
            // println!("\tX: {}, Y: {}", x, y);
            let pixel = font[index + y as usize] & (1 << (font_width - 1 - x));
            // println!("\tPixel: {}", pixel);
            let pixel_color = if pixel != 0 { Rgb([0, 0, 0]) } else { Rgb([255, 255, 255]) }; // Black or white based on font pixel
            // println!("\tPixel Color: {:?}", pixel_color);
            // println!("\tX Offset: {}, Y Offset: {}", x_offset + x as u32, y_offset + y as u32);
            // println!("\tDimensions: {:?}", image.dimensions());
            image.put_pixel(x_offset + x as u32, y_offset + y as u32, pixel_color);
        }
    }
}

/* fn draw_character(image: &mut RgbImage, character: char, x: u32, y: u32){
    let font = include_bytes!("../simple-8x16.font");
    let font_width = 8;
    let font_height = 16;
    let x_offset = x ; // Assuming each character occupies 8 pixels horizontally
    let y_offset = y ; // Assuming each character occupies 16 pixels vertically
    let color = Rgb([0, 0, 0]); // Black color for ASCII art

    // Map the ASCII character to its corresponding position in the font
    let index = (character as usize) * font_height;

    // Draw the character onto the image
    for y in 0..font_height {
        for x in 0..font_width {
            let pixel = font[index + y] & (1 << (font_width - 1 - x));
            let pixel_color = if pixel == 0 { Rgb([255, 255, 255]) } else { color };
            image.put_pixel(x_offset + x as u32, y_offset + y as u32, pixel_color);
        }
    }
} */