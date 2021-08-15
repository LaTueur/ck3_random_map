extern crate image as im;

use rand::seq::SliceRandom;
//use rand::Rng;
use rand::thread_rng;
use std::time::SystemTime;
use std::path::Path;
use std::fs;
use std::io::Write;

mod grid;
use grid::Grid;
use grid::GridVector;
mod numastype;
use numastype::NumAsType;

const NUM_OF_COLORS:u32 = (255 as u32).pow(3);
const LAND_MAP: &str = "from/big_plain.png";
const PROVINCE_GRID_SIZE:u32 = 32;
const LAND_COLOR: u8 = 10;
const BLACK:im::Rgb<u8> = im::Rgb([0, 0, 0]);
const WHITE:im::Rgb<u8> = im::Rgb([255, 255, 255]);
const PINK:im::Rgb<u8> = im::Rgb([255, 0, 128]);
const FOLDERS:[&str; 2] = ["map_data", "common/landed_titles"];
const ROOT_FOLDER:&str = "mod";

fn main() {
    let start_time = SystemTime::now();
    let mut map = im::open(LAND_MAP).unwrap().into_rgb8();
    let (width, height) = (map.width(), map.height());
    let mut colors: Vec<u32> = (0..NUM_OF_COLORS).collect();
    let mut rng = thread_rng();
    colors.shuffle(&mut rng);
    let mut map_pixels: Vec<bool> = vec!();
    let mut pixel_count: u32 = 0;
    for i in FOLDERS.iter(){
        let path = Path::new(ROOT_FOLDER).join(Path::new(i));
        fs::create_dir_all(path).unwrap();
    }
    map.save("mod/map_data/heightmap.png").unwrap();
    for pixel in map.pixels(){
        if pixel[0] > LAND_COLOR{
            map_pixels.push(true);
        }
        else{
            map_pixels.push(false);
        }
        pixel_count += 1;
    }
    for pixel in 0..pixel_count{
        let coords = pixel.as_coords(width);
        if map_pixels[pixel as usize]{
            map.put_pixel(coords.x, coords.y, WHITE);
        }
        else{
            map.put_pixel(coords.x, coords.y, PINK);
        }
    }
    map.save("mod/map_data/rivers.png").unwrap();
    let mut grids = Vec::<Grid>::collect_grids(width, height, &map_pixels, &colors);
    grids.pixels_to_provinces(width);
    for x in 0..width{
        for y in 0..height{
            map.put_pixel(x, y, BLACK);
        }
    }
    let mut definition = String::from("0;0;0;0;x;x;\n");
    let mut titles = String::from("e_test = { color = { 0 0 0 } color2 = { 255 255 255 } capital = c_test_1 k_test = { color = { 0 0 0 } color2 = { 255 255 255 \n");
    for grid in grids.iter(){
        if grid.province_pixels.len() == 0{
            continue
        }
        let color = grid.color;
        let index = grid.index+1;
        definition.push_str(&format!("{index};{};{};{};b_test_{index};x;\n", color[0], color[1], color[2], index=index));
        if grid.index%3 == 0{
            if grid.index != 0{
                titles.push_str(&"}");
            }
            if grid.index/3%3 == 0{
                titles.push_str(&format!(" }} \n d_test_{index} = {{ color = {{ {} {} {} }} color2 = {{ 255 255 255 }} capital = c_test_{index}\n", color[0], color[1], color[2], index=index));
            }
            titles.push_str(&format!("c_test_{index} = {{ color = {{ {} {} {} }} color2 = {{ 255 255 255 }}\n", color[0], color[1], color[2], index=index));
        }
        titles.push_str(&format!("b_test_{index} = {{ province = {index} color = {{ {} {} {} }} color2 = {{ 255 255 255 }} }}\n", color[0], color[1], color[2], index=index));
        for coords in grid.province_pixels.iter(){
            map.put_pixel(coords.x, coords.y, color);
        }
    }
    titles.push_str(&"} } } }");
    map.save("mod/map_data/provinces.png").unwrap();
    let mut file = fs::File::create("mod/map_data/definition.csv").unwrap();
    file.write_all(definition.as_bytes()).unwrap();
    let mut file = fs::File::create("mod/common/landed_titles/00_landed_titles.txt").unwrap();
    file.write_all(titles.as_bytes()).unwrap();

    println!("{}", SystemTime::now().duration_since(start_time).unwrap().as_millis().to_string());
}
