#[macro_use] extern crate log;
extern crate env_logger;
extern crate failure;
#[macro_use] extern crate clap;
extern crate rsgenetic;
extern crate image;
extern crate nsvg;
extern crate rand;

mod image_utils;
mod individual;
mod simulation;
mod error;
mod shape;
mod triangle;
mod rectangle;

use std::path::Path;
use std::process;
use std::sync::Arc;
use clap::{App, Arg};

fn main() {
    env_logger::init();

    let matches = App::new("geoshaper")
        .version(crate_version!())
        .author(crate_authors!(",\n"))
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .value_name("FILE")
                .help("target image")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("shape")
                .short("s")
                .long("shape")
                .value_name("STRING")
                .help("shape used to mimic image")
                .takes_value(true)
                .required(false)
        )
        .get_matches();

    let img_path = matches
        .value_of("image")
        .map(|istr| Path::new(istr))
        .unwrap();

    let img = image_utils::load_image(img_path).unwrap_or_else(|e| {
        eprintln!("opening image failed: {}", e);
        process::exit(1);
    });

    let shape = matches
        .value_of("shape")
        .unwrap_or("triangle");

    match simulation::run(Arc::new(img), simulation::Options { pop_size: 100, shape: shape.to_string() }) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("err: {}", e);
            process::exit(1);
        }
    }
}
