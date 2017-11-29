use std::path::Path;
use image_rs::{self, DynamicImage, ImageResult};

pub fn load_image(p: &Path) -> ImageResult<DynamicImage> {
    image_rs::open(p)
}
