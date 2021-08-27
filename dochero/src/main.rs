extern crate quick_xml;
extern crate zip;

mod doc;
mod utils;

use clap::{App, Arg};
use doc::Doc;

// Example
// dochero -i word.docx -o /home/mathuis/Documents/location
// dochero --input word.docx --output /home/mathuis/Documents/location
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Doc Hero")
        .about("Doc Hero converts your word document to doku wiki.")
        .arg(
            Arg::with_name("input")
                .short("-i")
                .long("--input")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("-o")
                .long("--output")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let raw_input_file = matches.value_of("input").expect("Input file is required");
    let raw_output_dir = matches
        .value_of("output")
        .expect("Output directory is required");

    if !utils::dir_exists(&raw_output_dir)? {
        println!("Directory does not exist.");
    }

    let doc = Doc::new(raw_input_file, raw_output_dir)?;
    doc.write_output()?;

    return Ok(());
}
