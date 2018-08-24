#[macro_use] extern crate clap;
extern crate env_logger;

extern crate geoshaper;

use geoshaper::simulation;

use std::path::Path;
use std::process;
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
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("render incremental rasters")
                .takes_value(false)
                .required(false)
        )
        .arg(
            Arg::with_name("maxiter")
                .short("mi")
                .long("maxiter")
                .value_name("INTEGER")
                .help("maximum number of iterations")
                .takes_value(true)
                .required(false)
        )
        .get_matches();

    let img_path = matches
        .value_of("image")
        .map(|istr| Path::new(istr))
        .unwrap();

    let mut options = simulation::Options::default();
    if matches.is_present("shape") {
        options.shape = matches.value_of("shape").unwrap().to_string();
    }
    if matches.is_present("maxiter") {
        options.max_iter = matches.value_of("maxiter").unwrap().parse().unwrap();
    }
    options.render_debug_rasters = matches.is_present("debug");

    match geoshaper::run(&img_path, Some(options)) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("err: {}", e);
            process::exit(1);
        }
    }
}
