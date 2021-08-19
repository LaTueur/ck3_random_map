use crate::numastype::NumAsType;
use crate::{LAND_COLOR};
use itertools::izip;

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
    fn biases(&self) -> [f64; 3]{
        match self {
            Terrain::Mountains => [0.9, 0.1, -0.1],
            Terrain::DesertMountain => [0.7, -0.5, 0.5],
            Terrain::Hills => [0.65, 0.1, -0.1],
            Terrain::Jungle => [0.0, 0.8, 0.5],
            Terrain::Drylands => [0.0, -0.1, 0.5],
            Terrain::Desert => [0.0, -0.6, 0.6],
            Terrain::Oasis => [0.0, 0.0, 0.0],
            Terrain::Floodplains => [0.0, 0.0, 0.0],
            Terrain::Plains => [-0.1, -0.2, 0.0],
            Terrain::Farmlands => [0.0, 0.0, 0.0],
            Terrain::Forest => [0.1, 0.1, -0.1],
            Terrain::Wetlands => [-0.3, 0.6, -0.1],
            Terrain::Steppe => [0.0, -0.2, -0.25],
            Terrain::Taiga => [0.0, -0.1, -0.8],
            Terrain::Ocean => [0.0, 0.0, 0.0]
        }
    }
    fn weights(&self) -> [f64; 3]{
        match self {
            Terrain::Mountains => [0.8, 0.1, 0.1],
            Terrain::DesertMountain => [0.7, 0.15, 0.15],
            Terrain::Hills => [0.7, 0.15, 0.15],
            Terrain::Jungle => [0.1, 0.5, 0.4],
            Terrain::Drylands => [0.1, 0.5, 0.4],
            Terrain::Desert => [0.1, 0.45, 0.45],
            Terrain::Oasis => [0.0, 0.0, 0.0],
            Terrain::Floodplains => [0.0, 0.0, 0.0],
            Terrain::Plains => [0.1, 0.3, 0.4],
            Terrain::Farmlands => [0.0, 0.0, 0.0],
            Terrain::Forest => [0.1, 0.4, 0.3],
            Terrain::Wetlands => [0.2, 0.6, 0.4],
            Terrain::Steppe => [0.1, 0.3, 0.6],
            Terrain::Taiga => [0.1, 0.2, 0.7],
            Terrain::Ocean => [5.0, 5.0, 5.0]
        }
    }
    pub fn calculate_likeliness(&self, values: [f64; 3]) -> f64{
        let mut likeliness = 0.0;
        for (value, bias, weight) in izip!(values, self.biases(), self.weights()){
            likeliness += (value-bias).powf(2.0)*weight
        }
        likeliness
    }
    pub fn all() -> Vec<Terrain>{
        vec!(
            Terrain::Mountains,
            Terrain::DesertMountain,
            Terrain::Hills,
            Terrain::Jungle,
            Terrain::Drylands,
            Terrain::Desert,
            //Terrain::Oasis,
            //Terrain::Floodplains,
            Terrain::Plains,
            //Terrain::Farmlands,
            Terrain::Forest,
            Terrain::Wetlands,
            Terrain::Steppe,
            Terrain::Taiga,
            Terrain::Ocean
        )
    }
}
impl std::fmt::Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct MapValue{total: u64, num: u32, max: u16, min: u16, average: u16}
impl MapValue{
    pub fn new() -> Self{
        MapValue{
            total: 0,
            num: 0,
            max: 0,
            min: 65535,
            average: 0
        }
    }
    pub fn apply_value(&mut self, value: u16){
        self.total += value as u64;
        self.num += 1;
        if value > self.max{
            self.max = value;
        }
        else if value < self.min{
            self.min = value;
        }
    }
    pub fn calculate_average(&mut self){
        self.average = (self.total / self.num as u64) as u16
    }
    pub fn normalize_value(&self, value: u16) -> f64{
        if value > self.average{
            (value-self.average) as f64/(self.max-self.average) as f64
        }
        else if value < self.average{
            -((self.average-value) as f64/(self.average-self.min) as f64)
        }
        else{
            0.0
        }
    }
}

pub trait TerrainVector{
    fn collect_terrain(height_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>,
        moisture_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>,
        temperature_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>) -> Self;
    fn calculate_map_values(height_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>,
        moisture_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>,
        temperature_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>) -> (MapValue, MapValue, MapValue);
    fn to_image(&self, width: u32) -> im::RgbImage;
    fn generate_gfx(&self, width: u32);
}
impl TerrainVector for Vec::<Terrain>{
    fn collect_terrain(height_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>,
        moisture_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>,
        temperature_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>) -> Self{
        let (width, height) = (height_map.width(), height_map.height());
        let mut map = vec!();
        let (elevation_value, moisture_value, temperature_value) = Vec::<Terrain>::calculate_map_values(height_map, moisture_map, temperature_map);
        
        for y in 0..height{
            for x in 0..width{
                let (elevation, moisture, temperature) =
                    (height_map.get_pixel(x, y)[0], moisture_map.get_pixel(x, y)[0], temperature_map.get_pixel(x, y)[0]);
                if elevation <= LAND_COLOR{
                    map.push(Terrain::Ocean);
                    continue
                }
                let normalized_values = [elevation_value.normalize_value(elevation), moisture_value.normalize_value(moisture), temperature_value.normalize_value(temperature)];
                map.push(*Terrain::all().iter().min_by(
                    |a, b|
                    a.calculate_likeliness(normalized_values)
                    .partial_cmp(&b.calculate_likeliness(normalized_values)).unwrap()
                ).unwrap());
            }
        }
        map
    }
    fn calculate_map_values(height_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>,
        moisture_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>,
        temperature_map: &im::ImageBuffer<im::Luma<u16>, Vec<u16>>) -> (MapValue, MapValue, MapValue){
        let (width, height) = (height_map.width(), height_map.height());
        let (mut elevation_value, mut moisture_value, mut temperature_value) = (MapValue::new(), MapValue::new(), MapValue::new());
        for y in 0..height{
            for x in 0..width{
                let (elevation, moisture, temperature) =
                    (height_map.get_pixel(x, y)[0], moisture_map.get_pixel(x, y)[0], temperature_map.get_pixel(x, y)[0]);
                if elevation <= LAND_COLOR{
                    continue
                }
                elevation_value.apply_value(elevation);
                moisture_value.apply_value(moisture);
                temperature_value.apply_value(temperature);
            }
        }
        elevation_value.calculate_average();
        moisture_value.calculate_average();
        temperature_value.calculate_average();
        (elevation_value, moisture_value, temperature_value)
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
        let terrains = Terrain::all();
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