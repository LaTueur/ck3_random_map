use crate::grid::Coords;

pub trait NumAsType{
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