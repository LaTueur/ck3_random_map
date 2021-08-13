extern crate image as im;

use std::cmp;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::thread_rng;
use std::time::SystemTime;

const NUM_OF_COLORS:u32 = (255 as u32).pow(3);
const LAND_MAP: &str = "mod/plain.png";
const PROVINCE_GRID_SIZE:u32 = 30;

#[derive(PartialEq)]
struct Coords{x: u32, y: u32}
impl Coords{
    fn simple_distance(&self, other: &Coords) -> u32{
        (cmp::max(self.x, other.x) - cmp::min(self.x, other.x))+(cmp::max(self.y, other.y) - cmp::min(self.y, other.y))
    }
    fn multi_distance(self: &Coords, coord2: &Coords, coord3: &Coords) -> u32{
        self.simple_distance(coord2) * self.simple_distance(coord3)
    }
}

trait NumToColor{
    fn as_rgb8(&self) -> im::Rgb<u8>;
}
impl NumToColor for u32{
    fn as_rgb8(&self) -> im::Rgb<u8>{
        im::Rgb([(self%255) as u8, (self/255%255) as u8, (self/255/255) as u8])
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
    for base_x in 0..width/PROVINCE_GRID_SIZE{
        for base_y in 0..height/PROVINCE_GRID_SIZE{
            let coords1 = Coords{x: rng.gen_range(0..PROVINCE_GRID_SIZE)+base_x*PROVINCE_GRID_SIZE,
                                y: rng.gen_range(0..PROVINCE_GRID_SIZE)+base_y*PROVINCE_GRID_SIZE};
            let coords2 = Coords{x: rng.gen_range(0..PROVINCE_GRID_SIZE)+base_x*PROVINCE_GRID_SIZE,
                                y: rng.gen_range(0..PROVINCE_GRID_SIZE)+base_y*PROVINCE_GRID_SIZE};
            provinces.push(vec!(coords1, coords2));
        }
    }
    for x in 0..width{
        for y in 0..height{
            let coords = Coords{x, y};
            provinces.iter_mut().min_by(|a, b|
                coords.multi_distance(&a[0], &a[1])
                .cmp(&coords.multi_distance(&b[0], &b[1])
            )).unwrap().push(coords);
        }
    }
    for i in 0..provinces.len(){
        let color = colors[i].as_rgb8();
        for coords in provinces[i].iter(){
            map.put_pixel(coords.x, coords.y, color);
        }
    }
    map.save("test.png").unwrap();
    println!("{}", SystemTime::now().duration_since(start_time).unwrap().as_millis().to_string());
}
