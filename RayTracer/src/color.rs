use image::RgbImage;
/// the multi-sample write_color() function
pub fn write_color(pixel_color: [u8; 3], img: &mut RgbImage, i: usize, j: usize) {
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb(pixel_color);
    // Write the translated [0,255] value of each color component.
}
