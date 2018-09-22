/// TODO new description

use std::collections::HashMap;


#[cfg(feature = "plane-0")]
/// Contains the raw file contents of the Unifont plane 0 font
const PLANE_0_RAW: &'static [u8] =
    include_bytes!("../data/unifont-11.0.02.hex.xz");

#[cfg(feature = "plane-1")]
/// Contains the raw file contents of the Unifont plane 1 font
const PLANE_1_RAW: &'static [u8] =
    include_bytes!("../data/unifont_upper-11.0.02.hex.xz");


/// Set to true after the first time a borrow of the font hashmap has been
/// requested.
static IS_INITIALISED: bool = false;


/// Contains parsed character definitions. A hashmap is utilised for fast
/// look-ups, since there are often gaps between defined characters in the
/// Unifont files.
static UNIFONT: HashMap<u32, FontChar> = HashMap::new();


/// Describes a single character in the font.
pub struct FontChar {
    /// Width, either 8px (half-width) or 16px (full-width)
    pub width: u8,
    /// Array of lines of the character's form. LSB is unused for half-width
    /// characters
    pub bitmap: [u16; 16]
}


/// Receives a raw (uncompressed) .hex font string, and adds each well-formed
/// character line to the `UNIFONT` hashmap.
fn initialise_generic(font_raw: &str) {
    
}


/// Called the first time that a borrow of the `UNIFONT` hashmap is requested,
/// in order to decompress and parse the embedded, xzipped .hex contents
fn initialise_unifont() {
    if cfg!(feature = "plane-0") {
        
    }

    if cfg!(feature = "plane-1") {
        
    }
}
