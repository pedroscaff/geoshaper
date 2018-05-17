use std::path::{Path, PathBuf};
use std::fmt;
use std::sync::Arc;
use std::fs::{create_dir, File};
use std::io::prelude::*;

use nsvg;
use image::{ColorType, DynamicImage, GenericImage, Rgba, RgbaImage};
use image::save_buffer;
use darwin_rs::Individual;

use image_utils::{image_diff, rgba_to_str};
use shape::{Polygon};
use error::Result;

#[derive(Clone)]
pub struct GImage {
    pub target: Arc<DynamicImage>,
    polygons: Vec<Polygon>,
    id: u32,
    width: u32,
    height: u32,
    avg_color: Rgba<u8>,
    path: PathBuf,
}

impl GImage {
    pub fn new(id: u32, i: Arc<DynamicImage>, avg_color: Rgba<u8>) -> Self {
        let (width, height) = i.dimensions();
        let polygons: Vec<Polygon> = Vec::new();
        GImage {
            target: i,
            polygons: polygons,
            width: width,
            height: height,
            id: id,
            avg_color: avg_color,
            path: PathBuf::from(format!("./tmp/{}", id)),
        }
    }

    fn random_shape(&self, polygon_type: &str) -> Polygon {
        Polygon::new(polygon_type, &self.width, &self.height)
    }

    fn svg_as_string(&self) -> String {
        let mut polygons = String::new();
        for polygon in &self.polygons {
            polygons.push_str(polygon.svg().as_str())
        }
        let mut svg =
            String::from(
                format!(
            "<svg width=\"{}\" height=\"{}\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">
            <rect width=\"{}\" height=\"{}\" x=\"0\" y=\"0\" fill=\"rgba({})\"/>",
            self.width, self.height, self.width, self.height, rgba_to_str(&self.avg_color)),
            );
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
        let new_shape = self.random_shape("triangle");
        self.polygons.push(new_shape);
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
        self.polygons.clear();
    }
}
