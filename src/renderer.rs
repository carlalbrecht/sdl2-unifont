use std::sync::MutexGuard;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use unifont;

/// Number of vertical pixels in each Unifont character.
const UNIFONT_HEIGHT: u32 = 16;

/// Storage class for rendering settings.
pub struct SurfaceRenderer {
    /// The colour to use to draw the text.
    pub fg_color: Color,
    /// The colour to use to fill the surface before drawing text.
    pub bg_color: Color,
    /// Integer scale multiplier, since Unifont is a raster font.
    pub scale: u32,
}

impl SurfaceRenderer {
    /// Creates a new Unifont renderer which renders text to new SDL surfaces.
    pub fn new(fg_color: Color, bg_color: Color) -> SurfaceRenderer {
        SurfaceRenderer {
            fg_color,
            bg_color,
            scale: 1,
        }
    }

    /// Draws the supplied text to a new surface which has been sized to fit the
    /// text exactly, using the renderer's style settings. Returns an `Err`
    /// result if a character was found which is not in the font, or the font
    /// could not be initialised.
    pub fn draw(&self, text: &str) -> Result<Surface, String> {
        let unifont = get_unifont()?;

        let mut surf = Surface::new(
            count_char_width(unifont, text)? * self.scale,
            UNIFONT_HEIGHT * self.scale,
            PixelFormatEnum::RGBA8888,
        )?;

        Ok(surf)
    }

    /// Sums the width of each character in the supplied text, and multiples the
    /// sum by the renderer's integer scale factor.
    pub fn measure_width(&self, text: &str) -> Result<u32, String> {
        Ok(self.scale * count_char_width(get_unifont()?, text)?)
    }

    /// May in the future take into consideration newlines and other formatting.
    /// For now, it just returns `16 * scale`, thus, the result of this method
    /// can always be safely `unwrap()`ped.
    pub fn measure_height(&self, _text: &str) -> Result<u32, String> {
        Ok(self.scale * 16)
    }
}

/// Maps `unifont`'s `Result` error type to ours, so that the `?` operator
/// can be utilised. The result of this should be passed to dependent
/// functions, rather than calling this function again, lest a deadlock
/// occur.
fn get_unifont<'a>() -> Result<MutexGuard<'a, unifont::FontChars>, String> {
    match unifont::get_unifont() {
        Ok(unifont) => Ok(unifont),
        Err(_) => {
            return Err("Failed to initialise embedded Unifont".to_string())
        }
    }
}

/// Finds the rendered width of a string, taking into consideration whether each
/// character is half-width (8px) or full-width (16px). Returns an error result
/// if a character is not found in the font (i.e. the feature to include it was
/// probably not enabled).
fn count_char_width(
    unifont: MutexGuard<unifont::FontChars>,
    text: &str,
) -> Result<u32, String> {
    let mut width_sum: u32 = 0;
    let iter = text.chars();

    for c in iter {
        match unifont.get(&(c as u32)) {
            None => {
                return Err(format!(
                    "Embedded Unifont does not contain {} (code point: 0x{:x})",
                    c, c as u32
                ))
            }

            Some(fc) => width_sum += fc.width as u32,
        }
    }

    Ok(width_sum)
}
