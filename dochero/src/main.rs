extern crate quick_xml;
extern crate zip;

use clap::{App, Arg};
// use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::str;
use zip::ZipArchive;

fn write_output(location: &str, output: String) -> Result<(), std::io::Error> {
    let mut handle = File::create(format!("{}/document.md", location).as_str())?;
    handle.write_all(output.as_bytes())?;

    return Ok(());
}

fn transform(output: &mut String, attributes: &mut HashMap<String, String>, value: &str) {
    //  Transform can be implemented for different types like: MD, DokuWiki, MediaWiki
    // When test is read, push current attributes

    let r = attributes.get("w:val");

    if r != None {
        let style = r.unwrap().as_str();
        match style {
            "Title" => {
                output.push_str(&format!("# {}\n", value));
            }
            "Subtitle" => {
                output.push_str(&format!("## {}\n", value));
            }
            "Heading1" => {
                output.push_str(&format!("### {}\n", value));
            }
            "Heading2" => {
                output.push_str(&format!("#### {}\n", value));
            }
            "Heading3" => {
                output.push_str(&format!("##### {}\n", value));
            }
            "TextBody" => {
                output.push_str(&format!("{}\n", value));
            }
            "InternetLink" => {
                output.push_str(&format!("[{}]\n", value));
            }
            _ => {}
        }
    }
}

fn parse(mut xml: quick_xml::Reader<&[u8]>) -> Result<String, quick_xml::Error> {
    let mut output = String::new();
    let mut buf = Vec::new();
    // let mut current_tag = String::new();
    let mut current_attributes = HashMap::new();

    loop {
        let event = xml.read_event(&mut buf);
        match event {
            Ok(Event::Empty(ref e)) => {
                // Unpack event and handle tag & attributes
                let current_tag = String::from_utf8_lossy(e.name()).to_string();

                // If the tag is a style tag, save the attributes to hashmap
                if current_tag.as_str() == "w:pStyle" || current_tag.as_str() == "w:rStyle" {
                    // println!("CURRENT TAG: {}", current_tag);

                    // Only clear the current attributes after the text is read
                    for a in e.attributes() {
                        let attribute = a?;
                        let key = str::from_utf8(&attribute.key)?;
                        let r = &attribute.unescaped_value()?;
                        let val = str::from_utf8(r)?;
                        current_attributes
                            .insert(String::from(key.clone()), String::from(val.clone()));
                    }
                }
            }
            Ok(Event::Text(ref e)) => {
                // Text(e) returns the text contained within the node
                let line = e.unescape_and_decode(&xml)?;
                let line = line.as_str();

                if !line.is_empty() {
                    transform(&mut output, &mut current_attributes, line);

                    // After text is pushed, clear current attributes
                    &current_attributes.clear();
                }
            }
            Ok(Event::Eof) => break,
            Err(err) => return Err(err),
            _ => continue,
        }
    }

    // After the output is created, pop the last `\n`
    output.pop();
    println!("{}", output);

    return Ok(output);
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

    let output = parse(xml)?;
    write_output(raw_output_dir, output)?;

    return Ok(());
}
