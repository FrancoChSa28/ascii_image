use std::fs;
use image::{imageops, open, GenericImageView, ImageBuffer, Luma, Pixel, Rgb, RgbImage};
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
    color: bool
}


fn main() {
    // env::set_var("RUST_BACKTRACE", "full");

    let args = Args::parse();
    let (width, contrast) = (args.width as i32, args.contrast as i32);
    let img_name = args.input.as_str();
    let color = args.color;

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

    

    // Save and open the ASCII art image.
    
    open::that("ascii_art.png").expect("Error opening image 'ascii_art.png'");

}

fn get_height<T>(img: &ImageBuffer<T, Vec<u8>>, width: i32) -> u32
where
    T: Pixel<Subpixel = u8> + 'static,
{
    let (orig_width, orig_height) = img.dimensions();
    let r = orig_height as f64 / orig_width as f64;
    (width as f64 * r * 0.5) as u32
}

// Generic function for both grayscale and color images
fn ascii_image<T>(
    height: u32,
    width: i32,
    density: &str,
    n: i32,
    process_pixel: impl Fn(&T) -> (u8, Rgb<u8>),
    img: &ImageBuffer<T, Vec<u8>>,
)
where
    T: Pixel<Subpixel = u8> + 'static, // Ensure pixel type has u8 subpixels and is static
    ImageBuffer<T, Vec<u8>>: GenericImageView<Pixel = T>, // Ensure it supports get_pixel
{
    let mut ascii_img = RgbImage::new(width as u32 * 8, height * 16);
    let font = include_bytes!("../simple-8x16.font");
    let font_width = 8;
    let font_height = 16;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x as u32, y);
            let (brightness, color) = process_pixel(pixel);

            let k = (brightness as f64 / 256.0 * n as f64).floor() as usize;
            let character = density.chars().nth((n - 1 - (k as i32)) as usize).unwrap_or(' ');

            let x_offset = x * 8;
            let y_offset = y * 16;

            draw_character(
                &mut ascii_img,
                character,
                font,
                font_width,
                font_height,
                x_offset as u32,
                y_offset,
                color != Rgb([0, 0, 0]),
                color,
            );
        }
    }

    ascii_img.save("ascii_art.png").expect("Error saving ASCII image");
    println!("Image saved successfully");
}

// Grayscale processing function
fn ascii_black_and_white(img_name: &str, width: i32, density: &str, n: i32) {
    // Read the image and convert it to grayscale.
    let mut img: ImageBuffer<Luma<u8>, Vec<u8>> = open(img_name).unwrap().to_luma8();
    let height = get_height(&mut img, width);
    let img = imageops::resize(&img, width as u32, height, image::imageops::FilterType::Lanczos3);

    // Save the grayscale image.
    img.save("grayscale.png").expect("Error saving grayscale image");

    ascii_image::<Luma<u8>>(
        height,
        width,
        density,
        n,
        |pixel| (pixel[0], Rgb([0, 0, 0])),  // Grayscale brightnessm
        &img
    );
}

// Color processing function
fn ascii_color(img_name: &str, width: i32, density: &str, n: i32) {
    // Read the image and convert it to grayscale.
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = open(img_name).unwrap().to_rgb8();
    let height = get_height(&mut img, width);
    let img = imageops::resize(&img, width as u32, height, image::imageops::FilterType::Lanczos3);

    // Save the grayscale image.
    img.save("colored.png").expect("Error saving colored image");

    ascii_image::<Rgb<u8>>(
        height,
        width,
        density,
        n,
        |pixel| {
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            let brightness = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
            (brightness, Rgb([r, g, b]))  // Return brightness + RGB color
        },
        &img
    );
}

// fn ascii_black_and_white(img_name: &str, width: i32, density: &str, n: i32) {
//     // Read the image and convert it to grayscale.
//     let mut img: ImageBuffer<Luma<u8>, Vec<u8>> = open(img_name).unwrap().to_luma8();
//     let height = get_height(&mut img, width);
//     let img = imageops::resize(&img, width as u32, height, image::imageops::FilterType::Lanczos3);

//     // Print some information about the image.
//     // println!("Image: {}x{} -> {}x{}", orig_width, orig_height, width, height);

//     // Save the grayscale image.
//     img.save("grayscale.png").expect("Error saving grayscale image");
    
//     // Create a new RGB image to store the ASCII art.
//     let mut ascii_img = RgbImage::new(width as u32 * 8, height * 16);
//     let font = include_bytes!("../simple-8x16.font");
//     let font_width = 8;
//     let font_height = 16;

//     // Map pixel brightness to ASCII characters.
//     for y in 0..height {
//         for x in 0..width {
//             let p = img.get_pixel(x as u32, y).0[0] as i32;
//             let k = (p as f64 / 256.0 * n as f64).floor() as usize;

//             let character = density.chars().nth((n - 1 - (k as i32) ) as usize).unwrap_or_else(|| ' ');

//             // Print p, k, character
//             // println!("y: {} \tx: {} \tp: {} \tk: {} \tchar: {} \tn: {}", y, x, p, k, character, n);
            
//             // print!("{}", character);

//             let x_offset = x * 8; // Each character occupies 8 pixels horizontally
//             let y_offset = y * 16; // Each character occupies 16 pixels vertically
//             draw_character(&mut ascii_img, character, font, font_width, font_height, x_offset as u32, y_offset, false, Rgb([0, 0, 0]));
//         }
//     }

//     // Save the ASCII art image.
//     ascii_img.save("ascii_art.png").expect("Error saving image 'ascii_art.png'");
//     println!("Image saved successfully");
// }

// fn ascii_color(img_name: &str, width: i32, density: &str, n: i32) {
//     let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = open(img_name).unwrap().to_rgb8(); // Read as color image
//     let height = get_height(&mut img, width);
//     let img = imageops::resize(&img, width as u32, height, image::imageops::FilterType::Lanczos3);

//     // Save the grayscale image.
//     img.save("colored.png").expect("Error saving grayscale image");

//     // Create a new RGB image to store the ASCII art.
//     let mut ascii_img = RgbImage::new(width as u32 * 8, height * 16);
//     let font = include_bytes!("../simple-8x16.font");
//     let font_width = 8;
//     let font_height = 16;

//     for y in 0..height {
//         for x in 0..width {
//             let pixel = img.get_pixel(x as u32, y);
//             let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
    
//             let brightness = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8; // Perceived brightness
//             let k = (brightness as f64 / 256.0 * n as f64).floor() as usize;
//             let character = density.chars().nth((n - 1 - (k as i32) ) as usize).unwrap_or(' ');
    
//             let x_offset = x * 8;
//             let y_offset = y * 16;

//             // print rgb
//             // println!("y: {} \tx: {} \tr: {} \tg: {} \tb: {} \tchar: {} \tn: {}", y, x, r, g, b, character, n);

//             draw_character(&mut ascii_img, character, font, font_width, font_height, x_offset as u32, y_offset, true, Rgb([r, g, b]));
//         }
//     }

//     // Save the ASCII art image.
//     ascii_img.save("ascii_art.png").expect("Error saving image 'ascii_art.png'");
//     println!("Image saved successfully");
// }

// Function to draw a character onto the image
fn draw_character(image: &mut RgbImage, character: char, font: &[u8], font_width: u32, font_height: u32, x_offset: u32, y_offset: u32, colored: bool, color: Rgb<u8>) {
    // Map the ASCII character to its corresponding position in the font
    let index = (character as usize) * (font_height as usize);
    // println!("Index: {}", index);
    // Draw the character onto the image
    for y in 0..font_height {
        for x in 0..font_width {
            // println!("\tX: {}, Y: {}", x, y);
            let pixel = font[index + y as usize] & (1 << (font_width - 1 - x));
            // println!("\tPixel: {}", pixel);
            let pixel_color = match colored {
                true => {
                    if pixel != 0 {
                        Rgb([color[0], color[1], color[2]]) // Use the specified color for the pixel
                    } else {
                        Rgb([255, 255, 255]) // White background
                    }
                },
                false => {
                    if pixel != 0 { Rgb([0, 0, 0]) } else { Rgb([255, 255, 255]) } // Black or white based on font pixel
                }
                
            };
            // let pixel_color = if pixel != 0 { Rgb([0, 0, 0]) } else { Rgb([255, 255, 255]) }; // Black or white based on font pixel
            // println!("\tPixel Color: {:?}", pixel_color);
            // println!("\tX Offset: {}, Y Offset: {}", x_offset + x as u32, y_offset + y as u32);
            // println!("\tDimensions: {:?}", image.dimensions());
            image.put_pixel(x_offset + x as u32, y_offset + y as u32, pixel_color);
        }
    }
}
