use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;


// Include generated font description file
include!(concat!(env!("OUT_DIR"), "/unifont.rs"));


/// Number of vertical pixels in each Unifont character.
const UNIFONT_HEIGHT: u32 = 16;


/// Storage class for rendering settings.
pub struct SurfaceRenderer {
    /// The colour to use to draw the text.
    pub fg_color: Color,
    /// The colour to use to fill the surface before drawing text.
    pub bg_color: Color,
    /// Integer scale multiplier, since Unifont is a raster font.
    pub scale: u32
}


impl SurfaceRenderer {
    /// Creates a new Unifont renderer which renders text to new SDL surfaces.
    pub fn new(fg_color: Color, bg_color: Color) -> SurfaceRenderer {
        SurfaceRenderer { fg_color, bg_color, scale: 1 }
    }


    /// Draws the supplied text to a new surface which has been sized to fit the
    /// text exactly, using the renderer's style settings.
    pub fn draw(&self, text: &str) -> Result<Surface, String> {
        let width = count_char_width(&text)?;

        let mut surf = Surface::new(width * self.scale,
                                    UNIFONT_HEIGHT * self.scale,
                                    PixelFormatEnum::RGBA8888)?;

        Ok(surf)
    }
}


/// Finds the rendered width of a string, taking into consideration whether each
/// character is half-width (8px) or full-width (16px). Returns an error result
/// if a character is not found in the font (i.e. the feature to include it was
/// probably not enabled).
fn count_char_width(text: &str) -> Result<u32, String> {
    let mut width_sum: u32 = 0;
    let iter = String::from(text).chars();

    for c in iter {
        match UNIFONT.get(&(c as u32)) {
            None => return Err(
                format!("Embedded Unifont does not contain {} (code: 0x{:x})",
                        c, c as u32)),

            Some(fc) => width_sum += fc.width as u32
        }
    }

    Ok(width_sum)
}
