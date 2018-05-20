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
use image_utils::{image_diff, rgba_to_str, image_area_diff};
use shape::{Polygon, Point};
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

pub trait Individual {
    fn mutate(&self, shape: Polygon, new_id: u32) -> GImage;
    fn fitness_full(&self) -> u64;
    fn fitness_mutation(&self) -> u64;
}

impl GImage {
    pub fn new(id: u32, i: Arc<DynamicImage>, avg_color: Rgba<u8>, width: u32, height: u32) -> Self {
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

    /**
     * @brief      returns the bounds of the last mutation
     *
     * @param      &self
     *
     * @return     &[Point]
     */
    pub fn mutation_area(&self) -> [Point; 2] {
        let last_polygon = self.polygons.last().unwrap();
        last_polygon.get_bounds()
    }

    pub fn get_last_polygon(&self) -> Polygon {
        self.polygons.last().unwrap().clone()
    }

    pub fn add_polygon(&mut self, polygon: Polygon) {
        self.polygons.push(polygon);
    }

    pub fn as_rgba_img(&self) -> Result<RgbaImage> {
        match self.save_svg() {
            Err(e) => {
                error!("error saving SVG of individual {}: {}", self.id, e);
                Err(e)
            },
            Ok(_) => {
                debug!("wrote SVG for individual {}", self.id);
                match self.raster() {
                    Ok(r) => Ok(r),
                    Err(e) => {
                        error!("error rasterizing individual {}: {}", self.id, e);
                        Err(e)
                    }
                }
            }
        }
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
            <rect width=\"{}\" height=\"{}\" x=\"0\" y=\"0\" fill=\"rgb({})\"/>",
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
    fn mutate(&self, mut candidate: Polygon, new_id: u32) -> GImage {
        let mut rng = thread_rng();
        let angle_generator = Range::new(0, 361);
        let angle = angle_generator.ind_sample(&mut rng) as f32;
        candidate.rotate(&angle);
        let mut v : Vec<Polygon> = self.polygons.clone();
        v.push(candidate);
        GImage {
            target: self.target.clone(),
            polygons: v,
            width: self.width,
            height: self.height,
            id: new_id,
            avg_color: self.avg_color,
            path: PathBuf::from(format!("./tmp/{}", new_id))
        }
    }

    fn fitness_mutation(&self) -> u64 {
        match self.save_svg() {
            Err(e) => {
                error!("error saving SVG of individual {}: {}", self.id, e);
                9999
            }
            Ok(_) => {
                debug!("wrote SVG for individual {}", self.id);
                match self.raster() {
                    Ok(r) => image_area_diff(self.target.clone(), &r, self.mutation_area()),
                    Err(e) => {
                        error!("error rasterizing individual {}: {}", self.id, e);
                        9999
                    }
                }
            }
        }
    }

    fn fitness_full(&self) -> u64 {
        image_diff(self.target.clone(), &self.as_rgba_img().unwrap())
    }
}
