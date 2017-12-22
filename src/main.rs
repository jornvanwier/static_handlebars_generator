extern crate clap;
extern crate rocket_contrib;
extern crate serde_json;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;

use clap::{App, Arg};
use rocket_contrib::Template;
use serde_json::Value;

fn main() {
    let matches = App::new("Static Handlebars Generator")
        .version("0.1")
        .author("Jorn van Wier <contact@jornvanwier.com>")
        .about("Renders templates using supplied data for usage in static websites.")
        .arg(
            Arg::with_name("template_dir")
                .short("t")
                .long("template_dir")
                .help("The directory to take templates from.")
                .index(1)
                .takes_value(true)
                .required(true))
        .arg(
            Arg::with_name("data_dir")
                .short("d")
                .long("data_dir")
                .help("The directory to take data (in json files) from.")
                .index(2)
                .takes_value(true)
                .required(true))
        .arg(
            Arg::with_name("file")
            .short("f")
            .long("file")
            .help("The file to render, other required files (e.g. partials) will be automatically included. Should be without any extensions. The program will look for a template and a date file with the approriate extensions in the template_dir and data_dir.")
                .index(3)
            .takes_value(true)
            .required(true))
        .arg(
            Arg::with_name("output_dir")
                .short("o")
                .long("output_dir")
                .help("The directory to place the finished file in. Will output to stdout otherwise.")
                .takes_value(true))
        .arg(
            Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Output information for every step."))
        .get_matches();

    let template_dir = matches.value_of("template_dir").expect("No template_dir");
    let data_dir = matches.value_of("data_dir").expect("No data_dir");
    let file = matches.value_of("file").expect("No file").to_string();
    let output_dir = matches.value_of("output_dir");
    let verbose = matches.is_present("verbose");

    let mut data_path = PathBuf::new();
    data_path.push(data_dir);
    data_path.push([&file, ".json"].concat());

    let mut data_file = File::open(&data_path).expect(&format!("Couldn't open data file {}", data_path.to_string_lossy()));
    let mut contents = String::new();
    data_file.read_to_string(&mut contents).expect(&format!("Couldn't read data file {}", data_path.to_string_lossy()));

    let context: Value = serde_json::from_str(&contents).expect(&format!("Couldn't parse data file {}", data_path.to_string_lossy()));

    println!("{} {} {}", template_dir, file, context);
    let output = Template::show(template_dir, file, context).expect("Couldn't render template");

    if let Some(output_dir) = output_dir {
        // write to file
    } else {
        print!("{}", output);
    }
}