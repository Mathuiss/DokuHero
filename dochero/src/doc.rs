use crate::utils;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::str;
use zip::ZipArchive;

pub struct Doc<'a> {
    output_dir: &'a str,
    output: String,
}

impl<'a> Doc<'a> {
    pub fn new(input_file: &'a str, output_dir: &'a str) -> Result<Doc<'a>, quick_xml::Error> {
        // Transform docx to XML
        let input_file = utils::locate_file(input_file).expect("The input file was not found.");
        let mut input_file =
            ZipArchive::new(input_file).expect("Zip archive was corrupted. Failed to read."); // Create zip archive
        let mut input_file = input_file
            .by_name("word/document.xml")
            .expect("Document does not container inner XML representation"); // Search for zip file by name

        // Reading contents of unzipped document.xml
        let mut input = String::new();
        input_file
            .read_to_string(&mut input)
            .expect("Inner XML document corrupted. Failed to read."); // Read file to String buffer

        Ok(Doc {
            output_dir: output_dir,
            output: Doc::parse(input)?,
        })
    }

    pub fn parse(input: String) -> Result<String, quick_xml::Error> {
        // XML vars
        let mut xml = Reader::from_reader(input.as_bytes()); // Parse XML file
        let mut buf = Vec::new();

        // dochero vars
        let mut output = String::new();
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
                        let md = Doc::transform(&mut current_attributes, line);
                        if md != None {
                            output.push_str(md.unwrap().as_str());
                        }

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

        println!("{}", output); // Remove this later

        return Ok(output);
    }

    fn transform(attributes: &mut HashMap<String, String>, value: &str) -> Option<String> {
        //  Transform can be implemented for different types like: MD, DokuWiki, MediaWiki
        // When test is read, push current attributes
        let r = attributes.get("w:val");
        if r != None {
            let style = r.unwrap().as_str();
            return match style {
                "Title" => Some(format!("# {}\n", value)),
                "Subtitle" => Some(format!("## {}\n", value)),
                "Heading1" => Some(format!("### {}\n", value)),
                "Heading2" => Some(format!("#### {}\n", value)),
                "Heading3" => Some(format!("##### {}\n", value)),
                "TextBody" => Some(format!("{}\n", value)),
                "InternetLink" => Some(format!("[{}]\n", value)),
                _ => None,
            };
        }

        None
    }

    // Getters and setters
    // pub fn get_output_dir(&self) -> &str {
    //     self.output_dir
    // }

    // pub fn set_output_dir(&mut self, output_dir: &'a str) {
    //     self.output_dir = output_dir;
    // }

    // pub fn get_output(&self) -> &String {
    //     &self.output
    // }

    // pub fn get_output_mut(&mut self) -> &String {
    //     &self.output
    // }

    // pub fn set_output(&mut self, output: String) {
    //     self.output = output;
    // }

    // Write the output to the output location held in self
    pub fn write_output(&self) -> Result<(), std::io::Error> {
        let mut handle = File::create(format!("{}/document.md", self.output_dir).as_str())?;
        handle.write_all(self.output.as_bytes())?;
        return Ok(());
    }
}
