#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
extern crate image;
extern crate nsvg;
extern crate rand;
extern crate scoped_threadpool;

mod error;
mod image_utils;
mod individual;
mod rectangle;
mod shape;
pub mod simulation;
mod triangle;

use std::path::Path;
use std::sync::Arc;

pub fn run(img_path: &Path, options: Option<simulation::Options>) -> error::Result<()> {
    let img = image_utils::load_image(img_path)?;

    simulation::run(Arc::new(img), options.unwrap_or_default())
}
