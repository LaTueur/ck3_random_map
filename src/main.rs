extern crate image as im;

use rand::seq::SliceRandom;
//use rand::Rng;
use rand::thread_rng;
use std::time::SystemTime;
use std::path::Path;
use std::fs;

mod grid;
use grid::Grid;
use grid::GridVector;
mod numastype;
use numastype::NumAsType;
mod noise;
mod terrain;
use crate::terrain::*;

const NUM_OF_COLORS:u32 = (255 as u32).pow(3);
const LAND_MAP: &str = "from/random.png";
const PROVINCE_GRID_SIZE:u32 = 64;
const LAND_COLOR: u16 = 4352;
const BLACK:im::Rgb<u8> = im::Rgb([0, 0, 0]);
const WHITE:im::Rgb<u8> = im::Rgb([255, 255, 255]);
const PINK:im::Rgb<u8> = im::Rgb([255, 0, 128]);
const LAND_FLAT_COLOR:im::Rgb<u8> = im::Rgb([170, 160, 140]);
const WATER_FLAT_COLOR:im::Rgb<u8> = im::Rgb([130, 130, 120]);
const FOLDERS:[&str; 4] = ["map_data", "common/landed_titles", "common/province_terrain", "gfx/map/terrain"];
const ROOT_FOLDER:&str = "mod";
const SEED: u32 = 9787;

fn main() {
    let start_time = SystemTime::now();
    let height_map = noise::generate_noise_map(1024, 512, 7, 2.12323, 0.5, 1.0, 3.0, -0.3, 10.0, SEED);
    //let height_map = im::open(LAND_MAP).unwrap().into_luma16();
    let (width, height) = (height_map.width(), height_map.height());
    let moisture_map = noise::generate_noise_map(width, height, 3, 2.02345, 0.5, 0.5, 3.0, -0.9, 3.5, SEED + 10);
    let temperature_map = noise::generate_noise_map(width, height, 3, 2.201348, 0.5, 0.5, 3.0, -0.9, 3.5, SEED + 15);
    let terrain_map = Vec::<Terrain>::collect_terrain(&height_map, &moisture_map, &temperature_map);
    let mut colors: Vec<u32> = (0..NUM_OF_COLORS).collect();
    let mut rng = thread_rng();
    colors.shuffle(&mut rng);
    let mut map_pixels: Vec<bool> = vec!();
    let mut pixel_count: u32 = 0;
    for i in FOLDERS.iter(){
        let path = Path::new(ROOT_FOLDER).join(Path::new(i));
        fs::create_dir_all(path).unwrap();
    }
    height_map.save("mod/map_data/heightmap.png").unwrap();
    moisture_map.save("mod/map_data/moisturemap.png").unwrap();
    temperature_map.save("mod/map_data/temperaturemap.png").unwrap();
    terrain_map.to_image(width).save("mod/map_data/terrainmap.png").unwrap();
    terrain_map.generate_gfx(width);
    for pixel in height_map.pixels(){
        if pixel[0] > LAND_COLOR{
            map_pixels.push(true);
        }
        else{
            map_pixels.push(false);
        }
        pixel_count += 1;
    }
    let mut map: im::RgbImage = im::ImageBuffer::new(width, height);
    for pixel in 0..pixel_count{
        let coords = pixel.as_coords(width);
        if map_pixels[pixel as usize]{
            map.put_pixel(coords.x, coords.y, LAND_FLAT_COLOR);
        }
        else{
            map.put_pixel(coords.x, coords.y, WATER_FLAT_COLOR);
        }
    }
    map.save("mod/gfx/map/terrain/flatmap.png").unwrap();
    let mut grids = Vec::<Grid>::collect_grids(width, height, &map_pixels, &colors);
    grids.pixels_to_provinces(width);
    for x in 0..width{
        for y in 0..height{
            map.put_pixel(x, y, BLACK);
        }
    }
    
    grids.save_to_files(width, &mut map, &terrain_map);

    println!("{}", SystemTime::now().duration_since(start_time).unwrap().as_millis().to_string());
}
