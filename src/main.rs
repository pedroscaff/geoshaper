extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("geoshaper")
    					.version("0.1.0")
    					.author("Pedro Scaff <email>")
    					.arg(Arg::with_name("image")
    						.short("i")
    						.long("image")
    						.value_name("FILE")
    						.help("target image")
    						.takes_value(true)
    						.required(true))
    					.get_matches();
    let img = matches.value_of("image").unwrap();
    println!("Target image -> {}", img);
}
