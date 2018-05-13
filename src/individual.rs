use std::path::Path;
use std::fmt;
use std::sync::Arc;
use image::{DynamicImage, GenericImage, Rgba, RgbaImage};
use darwin_rs::Individual;
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};
use std::fs::File;
use std::io::Result;
use std::io::prelude::*;
use nsvg;
use image_utils::{rgba_to_str, image_diff};

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
        write!(f, "{}", self.svg())
    }
}

impl Shape {
    pub fn svg(&self) -> String {
        let fill = if self.points[0].x % 2 == 0 {
            "white"
        } else {
            "gray"
        };
        format!("<polygon points=\"{},{} {},{} {},{}\" fill=\"{}\" />", 
            self.points[0].x, self.points[0].y, self.points[1].x, self.points[1].y, self.points[2].x, self.points[2].y, fill)
    }
}

#[derive(Clone)]
pub struct GImage {
    pub target: Arc<DynamicImage>,
    shapes: Vec<Shape>,
    id: u32,
    width: u32,
    height: u32,
    avg_color: Rgba<u8>
}

impl GImage {
    pub fn new(id: u32, i: Arc<DynamicImage>, avg_color: Rgba<u8>) -> Self {
        let (width, height) = i.dimensions();
        let shapes : Vec<Shape> = Vec::new();
        GImage {
            target: i,
            shapes: shapes,
            width: width,
            height: height,
            id: id,
            avg_color: avg_color
        }
    }

    fn random_shape(&self) -> Shape {
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
        Shape {
            points: points
        }
    }

    fn svg_as_string(&self) -> String {
        let mut polygons = String::new();
        for shape in &self.shapes {
            polygons.push_str(shape.svg().as_str())
        }
        let mut svg = String::from(format!(
            "<svg width=\"{}\" height=\"{}\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">
            <rect width=\"{}\" height=\"{}\" x=\"0\" y=\"0\" fill=\"{}\"/>",
            self.width, self.height, self.width, self.height, rgba_to_str(&self.avg_color)));
        svg.push_str(polygons.as_str());
        svg.push_str("</svg>");
        svg
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let mut file = File::create(path)?;
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
        let new_shape = self.random_shape();
        self.shapes.push(new_shape);
    }

    fn calculate_fitness(&mut self) -> f64 {
        let path_str = format!("tmp/{}", self.id);
        self.save(path_str.as_str()).unwrap();
        let path = Path::new(&path_str);
        let svg = nsvg::parse_file(path, nsvg::Units::Pixel, 96.0).unwrap();
        let scale = 1.0;
        let raster : RgbaImage = svg.rasterize(scale).unwrap();
        image_diff(self.target.clone(), &raster)
    }

    fn reset(&mut self) {
        self.shapes.clear();
    }
}
