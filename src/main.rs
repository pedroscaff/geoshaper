#[macro_use] extern crate clap;
extern crate image;
extern crate darwin_rs;
extern crate rand;
extern crate nsvg;

mod image_utils;
mod individual;
mod simulation;

use std::path::Path;
use std::process;
use std::sync::Arc;
use clap::{Arg, App};

fn main() {
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
        .get_matches();

    let img_path = matches
        .value_of("image")
        .map(|istr| Path::new(istr))
        .unwrap();

    let img = image_utils::load_image(img_path).unwrap_or_else(|e| {
        eprintln!("opening image failed: {}", e);
        process::exit(1);
    });

    simulation::run(Arc::new(img), simulation::Options { pop_size: 100 });
}
