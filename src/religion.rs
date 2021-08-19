fn color_average(value1: u8, value2: u8) -> u8{
    (value1 as u16 * value2 as u16 / 255) as u8
}
fn format_icon(path: &str, color: im::Rgb<u8>) -> im::RgbaImage{
    let mut icon = im::open(path).unwrap().into_rgba8();
    for pixel in icon.pixels_mut(){
        pixel.0 = [color_average(color[0], pixel[0]), color_average(color[1], pixel[1]), color_average(color[2], pixel[2]), pixel[3]];
    }
    icon
}