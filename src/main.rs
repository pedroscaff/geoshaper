extern crate clap;
extern crate image as image_rs;
extern crate darwin_rs;
extern crate svgdom;

mod image;
mod simulation;

use std::path::Path;
use std::process;
use std::sync::Arc;
use clap::{Arg, App};

fn main() {
    let matches = App::new("geoshaper")
        .version("0.1.0")
        .author("Pedro Scaff <pedro@scaff.me>")
        .author("Robert Günzler <r@gnzler.io>")
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

    let img = image::load_image(img_path).unwrap_or_else(|e| {
        eprintln!("opening image failed: {}", e);
        process::exit(1);
    });

    simulation::run(Arc::new(img), simulation::Options { pop_size: 100 });
}
