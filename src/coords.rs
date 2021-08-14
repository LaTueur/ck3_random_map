use std::cmp;

#[derive(PartialEq, Clone, Copy)]
pub struct Coords{pub x: u32, pub y: u32}
impl Coords{
    fn simple_distance(&self, other: &Coords) -> u32{
        (cmp::max(self.x, other.x) - cmp::min(self.x, other.x))+(cmp::max(self.y, other.y) - cmp::min(self.y, other.y))
    }
    pub fn multi_distance(self: &Coords, coords: &[Coords]) -> f64{
        let mut total_distance = 0.0;
        let low: f64 = 0.5;
        for (i, coord) in coords.iter().enumerate(){
            total_distance = total_distance + (self.simple_distance(coord) as f64 * low.powf(i as f64));
        }
        total_distance
    }
    pub fn as_index(&self, width: u32) -> u32{
        self.x + self.y*width
    }
}