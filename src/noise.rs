use noise::{Perlin, utils::PlaneMapBuilder, utils::NoiseMapBuilder, Seedable};

pub fn generate_noise_map(width: u32, height: u32, octaves: usize, lacunarity: f64, persistance: f64, scale: f64, sea_level: f64, seed: u32) -> im::RgbImage{
    let perlin = Perlin::default().set_seed(seed);
    let mut noisemaps = vec!();
    let mut amplitudes = vec!();
    let mut total_amplitude = 0.0;
    for i in 0..octaves{
        noisemaps.push(
            PlaneMapBuilder::new(&perlin)
                .set_size(width as usize, height as usize)
                .set_x_bounds(0.0, scale * lacunarity.powf( i as f64 ))
                .set_y_bounds(0.0, scale * lacunarity.powf( i as f64 ))
                .build()
        );
        amplitudes.push(persistance.powf( i as f64 ));
        total_amplitude += persistance.powf( i as f64 );
    }
    
    let mut image: im::RgbImage = im::ImageBuffer::new(width, height);
    let multiplier = 255.0 * (1.0 - sea_level) / 2.0 / total_amplitude;
    for y in 0..height{
        for x in 0..width{
            let mut value = 0.0;
            for i in 0..octaves{
                value += noisemaps[i].get_value(x as usize, y as usize) * amplitudes[i];
            }
            if value < sea_level{
                value = sea_level;
            }
            let color = im::Rgb([((value - sea_level) * multiplier).round() as u8; 3]);
            image.put_pixel(x, y, color);
        }
    }
    image
}