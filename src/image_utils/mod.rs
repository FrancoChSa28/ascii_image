use image::{ImageBuffer, Pixel};

pub fn get_height<T>(img: &ImageBuffer<T, Vec<u8>>, width: i32) -> u32
where
    T: Pixel<Subpixel = u8> + 'static,
{
    let (orig_width, orig_height) = img.dimensions();
    let r = orig_height as f64 / orig_width as f64;
    (width as f64 * r * 0.5) as u32
}
