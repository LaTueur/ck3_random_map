extern crate image as im;

use std::cmp;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::thread_rng;
//use std::time::SystemTime;

const NUM_OF_COLORS:u32 = (255 as u32).pow(3);
const LAND_MAP: &str = "mod/plain.png";
const PROVINCE_GRID_SIZE:u32 = 20;

trait NumToColor{
    fn as_rgb8(&self) -> im::Rgb<u8>;
}
impl NumToColor for u32{
    fn as_rgb8(&self) -> im::Rgb<u8>{
        im::Rgb([(self%255) as u8, (self/255%255) as u8, (self/255/255) as u8])
    }
}

fn get_distance((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> u64{
    (((cmp::max(x1, x2) - cmp::min(x1, x2))*(cmp::max(y1, y2) - cmp::min(y1, y2))) as f64).sqrt() as u64
}

fn main() {
    let mut map = im::open(LAND_MAP).unwrap().into_rgb8();
    let (width, height) = (map.width(), map.height());
    let mut colors: Vec<u32> = (0..NUM_OF_COLORS).collect();
    let mut rng = thread_rng();
    colors.shuffle(&mut rng);
    let mut provinces: Vec<Vec<(u32, u32)>> = vec!();
    for base_x in 0..width/PROVINCE_GRID_SIZE{
        for base_y in 0..height/PROVINCE_GRID_SIZE{
            let coords = (rng.gen_range(0..PROVINCE_GRID_SIZE)+base_x*PROVINCE_GRID_SIZE,
                            rng.gen_range(0..PROVINCE_GRID_SIZE)+base_y*PROVINCE_GRID_SIZE);
            provinces.push(vec!(coords));
        }
    }
    for x in 0..width{
        for y in 0..height{
            let coords = (x, y);
            provinces.sort_by(|a, b| get_distance(coords, a[0])
                    .cmp(&get_distance(coords, b[0])));
            provinces[0].push(coords);
        }
    }
    for i in 0..provinces.len(){
        let color = colors[i].as_rgb8();
        for (x, y) in provinces[i].iter(){
            map.put_pixel(*x, *y, color)
        }
    }
    map.save("test.png").unwrap();
}
