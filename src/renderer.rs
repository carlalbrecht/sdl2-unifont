extern crate sdl2;

use sdl2::pixels;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;


// Include generated font description file
include!(concat!(env!("OUT_DIR"), "/unifont.rs"));


/// Number of vertical pixels in each Unifont character.
const UNIFONT_HEIGHT: u32 = 16


/// Colo(u)r type alias. Color order is RGBA. Abstracts over sdl2::pixels::Color
/// in case other rendering APIs are supported in the future.
pub type Color = (u8, u8, u8, u8);


/// Storage class for rendering settings.
pub struct SDLRenderer {
    /// The colour to use to draw the text.
    fg_color: pixels::Color,
    /// The colour to use to fill the surface before drawing text.
    bg_color: pixels::Color,
    /// Integer scale multiplier, since Unifont is a raster font.
    pub scale: u16
}


impl SDLRenderer {
    /// Creates a new Unifont renderer which renders text to new SDL surfaces.
    pub fn new(fg_color: Color, bg_color: Color) -> SDLRenderer {
        SDLRenderer { fg_color, bg_color, scale: 1 }
    }


    /// Sets the text colour to use for future draw operations.
    pub fn set_foreground(&mut self, color: Color) {
        self.fg_color = pixels::Color::RGBA(color.0, color.1, color.2, color.3);
    }


    pub fn get_foreground(&self) -> Color {
        self.fg_color.rgba()
    }


    /// Sets the surface background colour to use for future draw operations.
    pub fn set_background(&mut self, color: Color) {
        self.bg_color = pixels::Color::RGBA(color.0, color.1, color.2, color.3);
    }


    pub fn get_background(&self) -> Color {
        self.bg_color.rgba();
    }


    /// Draws the supplied text to a new surface which has been sized to fit the
    /// text exactly, using the renderer's style settings.
    pub fn draw(&self, text: &str) -> Result<Surface, &'static str> {
        let width = count_char_width(&text)?;

        let mut surf = Surface::new(width * self.scale,
                                    UNIFONT_HEIGHT * self.scale,
                                    PixelFormatEnum::RGBA8888);

        Ok(surf)
    }
}


/// Finds the rendered width of a string, taking into consideration whether each
/// character is half-width (8px) or full-width (16px). Returns an error result
/// if a character is not found in the font (i.e. the feature to include it was
/// probably not enabled).
fn count_char_width(text: &str) -> Result<u32, &'static str> {
    // TODO iterate characters
}
