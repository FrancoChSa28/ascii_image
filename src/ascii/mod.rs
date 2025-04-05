use image::{imageops, open, GenericImageView, ImageBuffer, Luma, Pixel, Rgb, RgbImage};

use crate::image_utils::get_height;

pub fn ascii_image<T>(
    height: u32,
    width: i32,
    density: &str,
    n: i32,
    process_pixel: impl Fn(&T) -> (u8, Rgb<u8>),
    img: &ImageBuffer<T, Vec<u8>>,
)
where
    T: Pixel<Subpixel = u8> + 'static,
    ImageBuffer<T, Vec<u8>>: GenericImageView<Pixel = T>,
{
    let mut ascii_img = RgbImage::new(width as u32 * 8, height * 16);
    let font = include_bytes!("../../simple-8x16.font");
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

pub fn draw_character(
    image: &mut RgbImage,
    character: char,
    font: &[u8],
    font_width: u32,
    font_height: u32,
    x_offset: u32,
    y_offset: u32,
    colored: bool,
    color: Rgb<u8>,
) {
    let index = (character as usize) * (font_height as usize);
    for y in 0..font_height {
        for x in 0..font_width {
            let pixel = font[index + y as usize] & (1 << (font_width - 1 - x));
            let pixel_color = if colored {
                if pixel != 0 { color } else { Rgb([255, 255, 255]) }
            } else {
                if pixel != 0 { Rgb([0, 0, 0]) } else { Rgb([255, 255, 255]) }
            };
            image.put_pixel(x_offset + x, y_offset + y, pixel_color);
        }
    }
}

// Grayscale processing function
pub fn ascii_black_and_white(img_name: &str, width: i32, density: &str, n: i32) {
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
pub fn ascii_color(img_name: &str, width: i32, density: &str, n: i32) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Luma, Rgb};

    #[test]
    fn test_draw_character_bw() {
        let mut img = image::RgbImage::new(8, 16);
        let font = include_bytes!("../../simple-8x16.font");
        draw_character(&mut img, 'A', font, 8, 16, 0, 0, false, Rgb([0, 0, 0]));
        assert_eq!(img.get_pixel(0, 0)[0], 255); // top-left pixel is drawn in black
    }

    #[test]
    fn test_get_height_preserves_aspect_ratio() {
        let img: ImageBuffer<Luma<u8>, _> = ImageBuffer::from_fn(200, 100, |_, _| Luma([255]));
        let width = 100;
        let height = super::get_height(&img, width);
        assert_eq!(height, 25); // 100/200 = 0.5 → 100 * 0.5 * 0.5 = 25
    }
}