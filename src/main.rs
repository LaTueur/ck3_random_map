extern crate image as im;

use rand::seq::SliceRandom;
//use rand::Rng;
use rand::thread_rng;
use std::time::SystemTime;
use std::path::Path;
use std::fs;
use std::io::Write;

mod coords;
use coords::Coords;
mod numastype;
use numastype::NumAsType;

const NUM_OF_COLORS:u32 = (255 as u32).pow(3);
const LAND_MAP: &str = "from/heightmap.png";
const PROVINCE_GRID_SIZE:u32 = 30;
const LAND_COLOR: u8 = 0;
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
    let mut provinces: Vec<Vec<Coords>> = vec!();
    let mut map_pixels: Vec<bool> = vec!();
    let mut land_pixels: Vec<u32> = vec!();
    let mut pixel_count: u32 = 0;
    for i in FOLDERS.iter(){
        let path = Path::new(ROOT_FOLDER).join(Path::new(i));
        fs::create_dir_all(path).unwrap();
    }
    map.save("mod/map_data/heightmap.png").unwrap();
    for pixel in map.pixels(){
        if pixel[0] > LAND_COLOR{
            map_pixels.push(true);
            land_pixels.push(pixel_count);
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
    for base_x in 0..width/PROVINCE_GRID_SIZE{
        for base_y in 0..height/PROVINCE_GRID_SIZE{
            let mut valid_pixels:Vec<Coords> = vec!();
            for x in 0..PROVINCE_GRID_SIZE{
                for y in 0..PROVINCE_GRID_SIZE{
                    let coords = Coords{x: x+base_x*PROVINCE_GRID_SIZE, y: y+base_y*PROVINCE_GRID_SIZE};
                    let index = coords.as_index(width);
                    if map_pixels[index as usize]{
                        valid_pixels.push(coords);
                    }
                }
            }
            if valid_pixels.len() > 1{
                provinces.push(vec!(*valid_pixels.choose(&mut rng).unwrap(), *valid_pixels.choose(&mut rng).unwrap()));
            }
        }
    }
    for i in land_pixels{
        let coords = i.as_coords(width);
        provinces.iter_mut().min_by(|a, b|
            coords.multi_distance(&a[0], &a[1])
            .cmp(&coords.multi_distance(&b[0], &b[1]))
        ).unwrap().push(coords);
    }
    for x in 0..width{
        for y in 0..height{
            map.put_pixel(x, y, BLACK);
        }
    }
    let mut definition = String::from("0;0;0;0;x;x;\n");
    let mut titles = String::from("e_test = { color = { 0 0 0 } color2 = { 255 255 255 } capital = c_test_1 k_test = { color = { 0 0 0 } color2 = { 255 255 255 \n");
    for i in 0..provinces.len(){
        let color = colors[i].as_rgb8();
        let index = i+1;
        definition.push_str(&format!("{index};{};{};{};b_test_{index};x;\n", color[0], color[1], color[2], index=index));
        if i%3 == 0{
            if i != 0{
                titles.push_str(&"}");
            }
            if i/3%3 == 0{
                titles.push_str(&format!(" }} \n d_test_{index} = {{ color = {{ {} {} {} }} color2 = {{ 255 255 255 }} capital = c_test_{index}\n", color[0], color[1], color[2], index=index));
            }
            titles.push_str(&format!("c_test_{index} = {{ color = {{ {} {} {} }} color2 = {{ 255 255 255 }}\n", color[0], color[1], color[2], index=index));
        }
        titles.push_str(&format!("b_test_{index} = {{ province = {index} color = {{ {} {} {} }} color2 = {{ 255 255 255 }} }}\n", color[0], color[1], color[2], index=index));
        for coords in provinces[i].iter(){
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
