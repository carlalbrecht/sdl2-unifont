use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;


/// Added to generated unifont.rs before font character entries
const UNIFONT_RS_HEADER: &'static str = r##"
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;

use std::collections::HashMap;

lazy_static! {
    static ref UNIFONT: HashMap<u32, StructHereEventually> = {
        let mut ret = hashmap! {
"##;

/// Added to generated unifont.rs following font characters
const UNIFONT_RS_FOOTER: &'static str = r##"
    };
}
"##;


/// Takes the path to a font .hex file to read, and an open File to output
/// std::collections::HashMap<u32, FontChar> entries into, and converts each
/// character defined in the source .hex file into the format accepted by the
/// maplit library's hashmap! macro for defining hashmap literals.
fn write_font(dest: &mut File, source_path: &Path) -> std::io::Result<()> {
    // TODO all of this

    Ok(())
}


/// Creates unifont.rs and writes all constant content into it. Dispatches tasks
/// to write variable data to unifont.rs.
fn main() -> std::io::Result<()> {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let data_dir = Path::new(&project_dir).join("data");

    // Open unifont source file for code generation output
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("unifont.rs");
    let mut font_dest = File::create(&dest_path)?;

    // Write constant unifont.rs header
    font_dest.write_all(UNIFONT_RS_HEADER.as_bytes())?;

    // Write font data
    #[cfg(feature = "plane-0")]
    write_font(&mut font_dest, &data_dir.join("unifont-11.0.02.hex"))?;

    #[cfg(feature = "plane-1")]
    write(font(&mut font_dest, &data_dir.join("unifont_upper-11.0.02.hex")))?;

    // Write constant unifont.rs footer
    font_dest.write_all(UNIFONT_RS_FOOTER.as_bytes())?;

    Ok(())
}
