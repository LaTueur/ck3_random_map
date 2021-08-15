use std::cmp;
use crate::{BLACK, PROVINCE_GRID_SIZE};
use crate::numastype::NumAsType;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
pub struct Coords{pub x: u32, pub y: u32}
impl Coords{
    pub fn new(x: u32, y: u32) -> Self{
        Coords{x, y}
    }
    fn simple_distance(&self, other: &Coords) -> u32{
        (cmp::max(self.x, other.x) - cmp::min(self.x, other.x))+(cmp::max(self.y, other.y) - cmp::min(self.y, other.y))
    }
    pub fn multi_distance(self: &Coords, coords_vec: &Vec<Coords>) -> f64{
        let mut total_distance = 0.0;
        let low: f64 = 0.5;
        for (i, cords) in coords_vec.iter().enumerate(){
            total_distance = total_distance + (self.simple_distance(cords) as f64 * low.powf(i as f64));
        }
        total_distance
    }
    pub fn as_index(&self, width: u32) -> u32{
        self.x + self.y*width
    }
}
#[derive(PartialEq, Clone)]
pub struct Grid{
    pub starters: Vec<Coords>,
    pub land_pixels: Vec<Coords>,
    pub province_pixels: Vec<Coords>,
    pub color: im::Rgb<u8>,
    pub index: u32
}

impl Grid{
    pub fn new(starters: Vec<Coords>, land_pixels: Vec<Coords>, index: u32, colors: &Vec<u32>) -> Self{
        Grid{
            starters, land_pixels, province_pixels: vec!(), color: colors[index as usize].as_rgb8(), index
        }
    }
    pub fn empty(land_pixels: Vec<Coords>, index: u32) -> Self{
        Grid{
            starters: vec!(), land_pixels, province_pixels: vec!(), color: BLACK, index
        }
    }
    pub fn get_neighbours(&self, width: u32, grids: &Vec<Grid>) -> Vec<usize>{
        let mut neighbours: Vec<usize> = vec!();
        for x in 0..5{
            for y in 0..3{
                let index = (y - 1)*width as i32 +(x - 2)+self.index as i32;
                if index < 0 || index as usize >= grids.len(){
                    continue
                }
                if grids[index as usize].starters.len() == 0 {
                    continue
                }
                neighbours.push(index as usize)
            }
        }
        neighbours
    }
}

pub trait GridVector{
    fn collect_grids(width:u32, height:u32, map_pixels: &Vec<bool>, colors: &Vec<u32>) -> Self;
    fn pixels_to_provinces(&mut self, width: u32);
}
impl GridVector for Vec::<Grid>{
    fn collect_grids(width:u32, height:u32, map_pixels: &Vec<bool>, colors: &Vec<u32>) -> Self{
        let mut rng = thread_rng();
        let mut grids: Vec<Grid> = vec!();
        for base_y in 0..height/PROVINCE_GRID_SIZE{
            let offset = rng.gen_range(0..PROVINCE_GRID_SIZE) as i64;
            for base_x in 0..width/PROVINCE_GRID_SIZE+1{
                let mut land_pixels:Vec<Coords> = vec!();
                for x in 0..PROVINCE_GRID_SIZE{
                    let coord_x = (x+base_x*PROVINCE_GRID_SIZE) as i64 - offset;
                    if coord_x < 0 || coord_x >= width as i64{
                        continue
                    }
                    for y in 0..PROVINCE_GRID_SIZE{
                        let coords = Coords{x: coord_x as u32, y: y+base_y*PROVINCE_GRID_SIZE as u32};
                        let index = coords.as_index(width);
                        if map_pixels[index as usize]{
                            land_pixels.push(coords);
                        }
                    }
                }
                let index = Coords::new(base_x, base_y).as_index(width/PROVINCE_GRID_SIZE+1);
                if land_pixels.len() > 1{
                    grids.push(Grid::new(
                        vec!(*land_pixels.choose(&mut rng).unwrap(), *land_pixels.choose(&mut rng).unwrap(), *land_pixels.choose(&mut rng).unwrap(), *land_pixels.choose(&mut rng).unwrap()),
                        land_pixels,
                        index,
                        &colors
                    ));
                }
                else{
                    grids.push(Grid::empty(land_pixels, index));
                }
            }
        }
        grids
    }
    fn pixels_to_provinces(&mut self, width: u32){
        for i in 0..self.len(){
            if self[i].land_pixels.len() == 0{
                continue
            }
            let grid = self[i].clone();
            let neighbours = grid.get_neighbours(width/PROVINCE_GRID_SIZE+1, &self);
            if neighbours.len() == 0{
                continue
            }
            for coords in grid.land_pixels{
                let index = *neighbours.iter().min_by(|a, b|
                    coords.multi_distance(&self[**a].starters)
                    .partial_cmp(&coords.multi_distance(&self[**b].starters)).unwrap()
                ).unwrap();
                self[index].province_pixels.push(coords);  
            }
        }
    }
}