#[macro_use] extern crate log;
extern crate failure;
extern crate image;
extern crate nsvg;
extern crate rand;
extern crate scoped_threadpool;

pub mod simulation;
mod image_utils;
mod individual;
mod error;
mod shape;
mod triangle;
mod rectangle;

use std::sync::Arc;
use std::path::Path;

pub fn run(img_path: &Path, options: Option<simulation::Options>) -> error::Result<()> {
    let img = image_utils::load_image(img_path)?;

    simulation::run(Arc::new(img), options.unwrap_or_default())
}
