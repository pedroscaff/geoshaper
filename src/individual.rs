use std::path::{Path, PathBuf};
use std::fmt;
use std::sync::Arc;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use rand::thread_rng;
use rand::distributions::{IndependentSample, Range};

use nsvg;
use image::{ColorType, DynamicImage, GenericImage, Rgba, RgbaImage};
use image::save_buffer;
use darwin_rs::Individual;

use image_utils::{image_diff, rgba_to_str};
use error::Result;

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
    avg_color: Rgba<u8>,
    path: PathBuf,
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
            avg_color: avg_color,
            path: PathBuf::from(format!("./tmp/{}", id)),
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

    fn save_svg(&self) -> Result<()> {
        match create_dir(self.path.parent().unwrap()) {
            Err(e) => match e.kind() {
                ::std::io::ErrorKind::AlreadyExists => (),
                _ => Err(e)?,
            },
            Ok(_) => (),
        }
        let mut file = File::create(&self.path)?;
        let content = self.svg_as_string();
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn raster(&self) -> Result<RgbaImage> {
        let svg = nsvg::parse_file(&self.path, nsvg::Units::Pixel, 96.0)?;
        let scale = 1.0;
        Ok(svg.rasterize(scale)?)
    }

    pub fn save_raster(&self, path: &Path) -> Result<()> {
        Ok(save_buffer(
            path,
            &self.raster()?.into_raw(),
            self.width,
            self.height,
            ColorType::RGBA(8),
        )?)
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
        match self.save_svg() {
            Err(e) => {
                error!("error saving SVG of individual {}: {}", self.id, e);
                9999.0
            }
            Ok(_) => {
                debug!("wrote SVG for individual {}", self.id);
                match self.raster() {
                    Ok(r) => image_diff(self.target.clone(), &r),
                    Err(e) => {
                        error!("error rasterizing individual {}: {}", self.id, e);
                        9999.0
                    }
                }
            }
        }
    }

    fn reset(&mut self) {
        self.shapes.clear();
    }
}
