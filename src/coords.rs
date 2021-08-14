use std::cmp;

#[derive(PartialEq, Clone, Copy)]
pub struct Coords{pub x: u32, pub y: u32}
impl Coords{
    fn simple_distance(&self, other: &Coords) -> u32{
        (cmp::max(self.x, other.x) - cmp::min(self.x, other.x))+(cmp::max(self.y, other.y) - cmp::min(self.y, other.y))
    }
    pub fn multi_distance(self: &Coords, coord2: &Coords, coord3: &Coords) -> u32{
        self.simple_distance(coord2) * self.simple_distance(coord3)
    }
    pub fn as_index(&self, width: u32) -> u32{
        self.x + self.y*width
    }
}