use std::path::Path;
use std::fmt;
use std::sync::Arc;
use image_rs::{self, DynamicImage, GenericImage, ImageResult};
use darwin_rs::Individual;
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};
use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use image_rs::ColorType;
use image_rs::save_buffer;
use nsvg::{parse_file, Units};

pub fn load_image(p: &Path) -> ImageResult<DynamicImage> {
    image_rs::open(p)
}

#[derive(Debug, Clone)]
struct Point {
    x: u32,
    y: u32
}

#[derive(Debug, Clone)]
struct Shape {
    points: Vec<Point>
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "<polygon points=\"{},{} {},{} {},{}\" style=\"fill:lime;stroke:purple;stroke-width:1\" />", 
            self.points[0].x, self.points[0].y, self.points[1].x, self.points[1].y, self.points[2].x, self.points[2].y
        )
    }
}

#[derive(Clone)]
pub struct GImage {
    pub target: Arc<DynamicImage>,
    shapes: Vec<Shape>,
    id: u32,
    width: u32,
    height: u32
}

impl GImage {
    pub fn new(id: u32, i: Arc<DynamicImage>) -> Self {
        let (width, height) = i.dimensions();
        let shapes : Vec<Shape> = Vec::new();

        GImage {
            target: i,
            shapes: shapes,
            width: width,
            height: height,
            id: id
        }
    }

    fn svg_as_string(&self) -> String {
        let mut polygons = String::new();
        for shape in &self.shapes {
            polygons.push_str(format!("{}", shape).as_str())
        }
        let mut svg = String::from(format!(
            "<svg width='{}' height='{}' version='1.1' xmlns='http://www.w3.org/2000/svg'>",
            self.width, self.height));
        svg.push_str(polygons.as_str());
        svg.push_str("</svg>");
        svg
    }

    fn save(&self) -> Result<()> {
        let mut file = File::create(format!("tmp/{}", self.id))?;
        let content = self.svg_as_string();
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl fmt::Debug for GImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "i: {:?}", self.target.dimensions())
    }
}

impl Individual for GImage {
    fn mutate(&mut self) {
        let x_generator = Range::new(0, self.width + 1);
        let y_generator = Range::new(0, self.height + 1);
        let mut rng = thread_rng();
        let p1 = Point {
            x: x_generator.ind_sample(&mut rng),
            y: y_generator.ind_sample(&mut rng)
        };
        let p2 = Point {
            x: x_generator.ind_sample(&mut rng),
            y: y_generator.ind_sample(&mut rng)
        };
        let p3 = Point {
            x: x_generator.ind_sample(&mut rng),
            y: y_generator.ind_sample(&mut rng)
        };
        let points = vec![p1, p2, p3];
        let new_shape = Shape {
            points: points
        };
        self.shapes.push(new_shape);
    }

    fn calculate_fitness(&mut self) -> f64 {
        self.save().unwrap();
        let path_str = format!("tmp/{}", self.id);
        let path = Path::new(&path_str);
        let svg = parse_file(path, Units::Pixel, 96.0).unwrap();
        let scale = 1.0;
        let raster = svg.rasterize(scale).unwrap();
        save_buffer(Path::new(&format!("tmp/r-{}.png", self.id)), &raster.into_raw(), self.width, self.height, ColorType::RGBA(8)).unwrap();
        0.0
    }

    fn reset(&mut self) {}
}
