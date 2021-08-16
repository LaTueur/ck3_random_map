use noise::{utils::PlaneMapBuilder, utils::NoiseMapBuilder, MultiFractal, Fbm};

pub fn generate_noise_map(width: u32, height: u32, octaves: usize, lacunarity: f64, persistance: f64, frequency: f64, scale: f64, sea_level: f64, seed: u32) -> im::RgbImage{
    let fbm = Fbm::new(seed)
        .set_frequency(frequency)
        .set_persistence(persistance)
        .set_lacunarity(lacunarity)
        .set_octaves(octaves);

    let result = PlaneMapBuilder::new(&fbm)
        .set_size(width as usize, height as usize)
        .set_x_bounds(0.0, scale)
        .set_y_bounds(0.0, scale)
        .build();
    
    let mut image: im::RgbImage = im::ImageBuffer::new(width, height);
    let multiplier = 255.0 * (1.0 - sea_level) / 5.0;
    for y in 0..height{
        for x in 0..width{
            let mut value = result.get_value(x as usize, y as usize);
            if value < sea_level{
                value = sea_level;
            }
            let color = im::Rgb([((value - sea_level) * multiplier).round() as u8; 3]);
            image.put_pixel(x, y, color);
        }
    }
    image
}