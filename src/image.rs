use std::path::Path;
use std::fmt;
use std::sync::Arc;
use image_rs::{self, DynamicImage, GenericImage, ImageResult};
use darwin_rs::Individual;
use svgdom::{Document, Node};

pub fn load_image(p: &Path) -> ImageResult<DynamicImage> {
    image_rs::open(p)
}

#[derive(Clone)]
pub struct GImage {
    pub target: Arc<DynamicImage>,
    pub canvas: String,
}

impl GImage {
    pub fn new(i: Arc<DynamicImage>) -> Self {
        let (x, y) = i.dimensions();
        let canvas = Document::from_str(&format!(
            "<svg width='{}' height='{}' version='1.1' xmlns='http://www.w3.org/2000/svg'></svg>",
            x,
            y
        )).unwrap();

        GImage {
            target: i,
            canvas: canvas.to_string(),
        }
    }
}

impl fmt::Debug for GImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "i: {:?}", self.target.dimensions())
    }
}

impl Individual for GImage {
    fn mutate(&mut self) {}

    fn calculate_fitness(&mut self) -> f64 {
        0.0
    }

    fn reset(&mut self) {}
}
