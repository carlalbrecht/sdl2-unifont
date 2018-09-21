extern crate handlebars;

use std::env;
use std::fs::File;
use std::path::Path;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Error, ErrorKind};

use handlebars::Handlebars;


#[allow(dead_code)]
const PLANE_0_FILENAME: &'static str = "unifont-11.0.02.hex";

#[allow(dead_code)]
const PLANE_1_FILENAME: &'static str = "unifont_upper-11.0.02.hex";


/// Reads a specified .hex font file and generates a FontChar for each character
/// in the font file.
fn gen_font_entries_for(font_file: &Path) -> std::io::Result<String> {
    let mut font_entries = String::new();
    let font = File::open(font_file)?;
    let reader = BufReader::new(font);
    let lines = reader.lines();

    // Returned as the Result value if the .hex file is invalid somehow
    let corrupt_file_err =
        Err(Error::new(ErrorKind::InvalidData, ".hex file malformed"));

    for l in lines {
        let mut font_entry = String::new();

        let line = l.unwrap();
        let mut split = line.split(":");

        // Read character code point
        let codepoint = match split.next() {
            Some(cp) => cp,
            None => return corrupt_file_err
        };

        // Read hex-encoded bitmap (not converted here)
        let bitmap = match split.next() {
            Some(bmp) => bmp,
            None => return corrupt_file_err
        };

        // Append code point to character line
        font_entry.push_str(codepoint);
        font_entry.push_str(" => { ");

        // Determine if we are dealing with a half or full-width character
        let char_width = bitmap.chars().count() / 4;

        if char_width != 8 && char_width != 16 {
            return corrupt_file_err;
        }

        // Append character width to character line
        font_entry.push_str(&format!("width: {}, bitmap: [", char_width));

        // Number of chars per row
        let row_width = char_width / 4;

        // Populate bitmap array
        for i in 0..bitmap.chars().count() / row_width {
            let line = &bitmap[i..i + row_width];

            if row_width == 2 {
                font_entry.push_str(&format!("0x{}00, ", line));
            } else {
                font_entry.push_str(&format!("0x{}, ", line));
            }
        }

        // Finish entry line and add to font entries string
        font_entry.push_str("] },\n");
        font_entries.push_str(&font_entry);
    }

    Ok(font_entries)
}


/// Generates the hashmap entries for all font characters in the Unicode layers
/// which have been selected using dependency features.
fn gen_font_entries(data_dir: &Path) -> std::io::Result<String> {
    let mut font_entries = String::new();

    #[cfg(feature = "plane-0")]
    font_entries.push_str(
        &gen_font_entries_for(&data_dir.join(PLANE_0_FILENAME))?);

    #[cfg(feature = "plane-1")]
    font_entries.push_str(
        &gen_font_entries_for(&data_dir.join(PLANE_1_FILENAME))?);

    Ok(font_entries)
}


/// Creates unifont.rs and writes all constant content into it. Dispatches tasks
/// to write variable data to unifont.rs.
fn main() -> std::io::Result<()> {
    let handlebars = Handlebars::new();

    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let data_dir = Path::new(&project_dir).join("data");
    let src_dir = Path::new(&project_dir).join("src");

    // Open unifont source file for code generation output
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("unifont.rs");
    let font_dest = File::create(&dest_path)?;

    // Generate font data
    let mut data = BTreeMap::new();
    data.insert("characters".to_string(), gen_font_entries(&data_dir)?);

    // Open unifont.rs template file
    let mut unifont_template = File::open(&src_dir.join("unifont.rs.template"))?;

    // Replace {{ characters }} handlebar with character data
    handlebars.render_template_source_to_write(&mut unifont_template, &data, font_dest).unwrap();

    Ok(())
}
