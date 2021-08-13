extern crate image as im;

use std::cmp;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::thread_rng;
use std::time::SystemTime;

const NUM_OF_COLORS:u32 = (255 as u32).pow(3);
const LAND_MAP: &str = "mod/heightmap.png";
const PROVINCE_GRID_SIZE:u32 = 30;
const LAND_COLOR: u8 = 16;
const BLACK:im::Rgb<u8> = im::Rgb([0, 0, 0]);
const WHITE:im::Rgb<u8> = im::Rgb([255, 255, 255]);
const PINK:im::Rgb<u8> = im::Rgb([255, 0, 128]);

#[derive(PartialEq, Clone, Copy)]
struct Coords{x: u32, y: u32}
impl Coords{
    fn simple_distance(&self, other: &Coords) -> u32{
        (cmp::max(self.x, other.x) - cmp::min(self.x, other.x))+(cmp::max(self.y, other.y) - cmp::min(self.y, other.y))
    }
    fn multi_distance(self: &Coords, coord2: &Coords, coord3: &Coords) -> u32{
        self.simple_distance(coord2) * self.simple_distance(coord3)
    }
    fn as_index(&self, width: u32) -> u32{
        self.x + self.y*width
    }
}

trait NumAsType{
    fn as_rgb8(&self) -> im::Rgb<u8>;
    fn as_coords(&self, width: u32) -> Coords;
}
impl NumAsType for u32{
    fn as_rgb8(&self) -> im::Rgb<u8>{
        im::Rgb([(self%255) as u8, (self/255%255) as u8, (self/255/255) as u8])
    }
    fn as_coords(&self, width: u32) -> Coords{
        Coords{x: self%width, y: self/width}
    }
}

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
    map.save("rivers.png").unwrap();
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
    for i in 0..provinces.len(){
        let color = colors[i].as_rgb8();
        for coords in provinces[i].iter(){
            map.put_pixel(coords.x, coords.y, color);
        }
    }
    map.save("provinces.png").unwrap();

    println!("{}", SystemTime::now().duration_since(start_time).unwrap().as_millis().to_string());
}
