use std::path::Path;
use image_rs::{self, DynamicImage, GenericImage, ImageResult};
use darwin_rs::Individual;
use svg::Document;

pub fn load_image(p: &Path) -> ImageResult<DynamicImage> {
    image_rs::open(p)
}

pub struct GImage {
    pub target: DynamicImage,
    pub canvas: Document,
}

impl GImage {
    pub fn new(i: DynamicImage) -> Self {
        GImage {
            target: i.clone(),
            canvas: Document::new().set("viewBox", (0, 0, i.dimensions().0, i.dimensions().1)),
        }
    }
}

impl Individual for GImage {
    fn mutate(&mut self) {
        unimplemented!()
    }

    fn calculate_fitness(&mut self) -> f64 {
        unimplemented!()
    }

    fn reset(&mut self) {
        unimplemented!()
    }
}
