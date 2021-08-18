use crate::numastype::NumAsType;
use crate::{LAND_COLOR};

const LUMA_WHITE: im::Luma<u8> = im::Luma([255]);

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Terrain {
    Mountains,
    DesertMountain,
    Hills,
    Jungle,
    Drylands,
    Desert,
    Oasis,
    Floodplains,
    Plains,
    Farmlands,
    Forest,
    Wetlands,
    Steppe,
    Taiga,
    Ocean
}
impl Terrain{
    pub fn color(&self) -> im::Rgb<u8>{
        match self {
            Terrain::Mountains => im::Rgb([100, 100, 100]),
            Terrain::DesertMountain => im::Rgb([23, 19, 38]),
            Terrain::Hills => im::Rgb([90, 50, 12]),
            Terrain::Jungle => im::Rgb([10, 60, 35]),
            Terrain::Drylands => im::Rgb([220, 45, 120]),
            Terrain::Desert => im::Rgb([225, 255, 0]),
            Terrain::Oasis => im::Rgb([194, 204, 143]),
            Terrain::Floodplains => im::Rgb([200, 50, 100]),
            Terrain::Plains => im::Rgb([204, 163, 102]),
            Terrain::Farmlands => im::Rgb([255, 0, 0]),
            Terrain::Forest => im::Rgb([50, 255, 25]),
            Terrain::Wetlands => im::Rgb([75, 200, 200]),
            Terrain::Steppe => im::Rgb([200, 100, 25]),
            Terrain::Taiga => im::Rgb([0, 90, 0]),
            Terrain::Ocean => im::Rgb([0, 0, 255])
        }
    }
    pub fn file(&self) -> &str{
        match self {
            Terrain::Mountains => "mountain_02_mask.png",
            Terrain::DesertMountain => "mountain_02_desert_mask.png",
            Terrain::Hills => "hills_01_mask.png",
            Terrain::Jungle => "forest_jungle_01_mask.png",
            Terrain::Drylands => "drylands_01_mask.png",
            Terrain::Desert => "desert_01_mask.png",
            Terrain::Oasis => "oasis_mask.png",
            Terrain::Floodplains => "floodplains_01_mask.png",
            Terrain::Plains => "plains_01_mask.png",
            Terrain::Farmlands => "farmland_01_mask.png",
            Terrain::Forest => "forest_leaf_01_mask.png",
            Terrain::Wetlands => "wetlands_02_mask.png",
            Terrain::Steppe => "steppe_01_mask.png",
            Terrain::Taiga => "forest_pine_01_mask.png",
            Terrain::Ocean => "beach_02_mask.png"
        }
    }
}
impl std::fmt::Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait TerrainVector{
    fn collect_terrain(height_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>,
        moisture_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>,
        temperature_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>) -> Self;
    fn to_image(&self, width: u32) -> im::RgbImage;
    fn generate_gfx(&self, width: u32);
}
impl TerrainVector for Vec::<Terrain>{
    fn collect_terrain(height_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>,
        moisture_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>,
        temperature_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>) -> Self{
        let (width, height) = (height_map.width(), height_map.height());
        let mut map = vec!();
        for y in 0..height{
            for x in 0..width{
                let (elevation, moisture, temperature) =
                    (height_map.get_pixel(x, y)[0], moisture_map.get_pixel(x, y)[0], temperature_map.get_pixel(x, y)[0]);
                let terrain = match elevation {
                    0..=LAND_COLOR => Terrain::Ocean,
                    0..=7000 => {
                        match temperature{
                            0..=8000 => {
                                Terrain::Taiga
                            }
                            8001..=20000 => {
                                match moisture{
                                    0..=30000 => {
                                        Terrain::Steppe
                                    }
                                    30001..=50000 => {
                                        Terrain::Forest
                                    }
                                    _ => {
                                        Terrain::Wetlands
                                    }
                                }
                            }
                            20001..=40000 => {
                                match moisture{
                                    0..=3000 => {
                                        Terrain::Drylands
                                    }
                                    3001..=28000 | 29001..=32000=> {
                                        Terrain::Plains
                                    }
                                    28001..=29000 => {
                                        Terrain::Farmlands
                                    }
                                    32001..=50000 => {
                                        Terrain::Forest
                                    }
                                    _ => {
                                        Terrain::Wetlands
                                    }
                                }
                            }
                            _ => {
                                match moisture{
                                    0..=5000 => {
                                        Terrain::Desert
                                    }
                                    5001..=40000 => {
                                        Terrain::Drylands
                                    }
                                    _ =>{
                                        Terrain::Jungle
                                    }

                                }
                            }
                        }
                    }
                    7001..=9000 => {
                        Terrain::Hills
                    }
                    _ => {
                        if moisture < 5000 && temperature > 40000{
                            Terrain::DesertMountain
                        }
                        else{
                            Terrain::Mountains
                        }
                    }
                };
                map.push(terrain);
            }
        }
        map
    }
    fn to_image(&self, width: u32) -> im::RgbImage{
        let mut image = im::RgbImage::new(width, self.len() as u32/width);
        for (i, terrain) in self.iter().enumerate(){
            let coords = (i as u32).as_coords(width);
            image.put_pixel(coords.x, coords.y, terrain.color());
        }
        image
    }
    fn generate_gfx(&self, width: u32){
        let height = self.len() as u32 / width;
        let terrains = collect_terrain_types();
        let mut images = vec!();
        for _terrain in terrains.iter(){
            let image: im::ImageBuffer<im::Luma<u8>, Vec<u8>> = im::ImageBuffer::new(width, height);
            images.push(image);
        }
        for (i, terrain) in self.iter().enumerate(){
            let index = terrains.iter().position(|&x| x == *terrain).unwrap();
            let coords = (i as u32).as_coords(width);
            images[index].put_pixel(coords.x, coords.y, LUMA_WHITE);
        }
        for (terrain, image) in terrains.iter().zip(images.iter()){
            image.save("mod/gfx/map/terrain/".to_owned() + terrain.file()).unwrap();
        }
    }
}
pub fn collect_terrain_types() -> Vec<Terrain>{
    vec!(
        Terrain::Mountains,
        Terrain::DesertMountain,
        Terrain::Hills,
        Terrain::Jungle,
        Terrain::Drylands,
        Terrain::Desert,
        Terrain::Oasis,
        Terrain::Floodplains,
        Terrain::Plains,
        Terrain::Farmlands,
        Terrain::Forest,
        Terrain::Wetlands,
        Terrain::Steppe,
        Terrain::Taiga,
        Terrain::Ocean
    )
}