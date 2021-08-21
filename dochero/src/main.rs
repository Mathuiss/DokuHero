extern crate quick_xml;
extern crate zip;

use clap::{App, Arg};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

fn parse(mut xml: quick_xml::Reader<&[u8]>) -> Result<String, quick_xml::Error> {
    let mut event_counter = 0;
    let mut output = Vec::new();
    let mut buf = Vec::new();

    loop {
        let xml_event = xml.read_event(&mut buf)?;
        match xml_event {
            Event::Start(_) => event_counter += 1,
            Event::Text(e) => output.push(e.unescape_and_decode(&xml)?),
            Event::Eof => break,
            _ => println!("Unhandled XML event"),
        }
    }

    for s in output {
        println!("{}", s);
    }

    return Ok(String::new());
}

fn locate_file(raw_input: &str) -> Result<File, std::io::Error> {
    if raw_input.starts_with('/') {
        return File::open(raw_input);
    } else {
        let cwd = current_dir()?;
        let cwd = cwd.to_str().unwrap();
        return File::open(format!("{}/{}", cwd, raw_input));
    }
}

fn dir_exists(raw_input: &str) -> Result<bool, std::io::Error> {
    if raw_input.starts_with('/') {
        return Ok(Path::new(raw_input).exists());
    } else {
        let cwd = current_dir()?;
        let cwd = cwd.to_str().unwrap();
        return Ok(Path::new(&format!("{}/{}", cwd, raw_input)).exists());
    }
}

// Example
// dochero -i word.docx -o /home/mathuis/Documents/location
// dochero --input word.docx --output /home/mathuis/Documents/location
fn main() -> Result<(), std::io::Error> {
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

    if !dir_exists(&raw_output_dir)? {
        println!("Directory does not exist.");
    }

    // Transform docx to XML
    let input_file = locate_file(raw_input_file)?;
    let mut input_file = ZipArchive::new(input_file)?; // Create zip archive
    let mut input_file = input_file.by_name("word/document.xml")?; // Search for zip file by name

    // Reading contents of unzipped document.xml
    let mut input = String::new();
    input_file.read_to_string(&mut input)?; // Read file to String buffer
    let xml = Reader::from_str(input.as_str()); // Parse XML file

    let output = parse(xml);

    // let document = build_document(input);
    return Ok(());
}
