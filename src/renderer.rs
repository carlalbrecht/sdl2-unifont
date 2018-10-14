use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

use bit_field::BitField;

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
    /// Whether or not to make text bold. Uses XTerm-style bolding, where the
    /// text is just drawn twice on the x-axis, one pixel apart
    pub bold: bool,
    /// Whether or not to make text italicised. Simply shifts pixels to the
    /// right every two vertical pixels.
    pub italic: bool,
}

impl SurfaceRenderer {
    /// Creates a new Unifont renderer which renders text to new SDL surfaces.
    pub fn new(fg_color: Color, bg_color: Color) -> SurfaceRenderer {
        SurfaceRenderer {
            fg_color,
            bg_color,
            scale: 1,
            bold: false,
            italic: false,
        }
    }

    /// Draws the supplied text to a new surface which has been sized to fit the
    /// text exactly, using the renderer's style settings. Returns an `Err`
    /// result if a character was found which is not in the font, or the font
    /// could not be initialised.
    pub fn draw(&self, text: &str) -> Result<Surface, String> {
        // Create new surface sized to text
        let width = count_char_width(text)? * self.scale;
        let mut surf = Surface::new(
            width,
            UNIFONT_HEIGHT * self.scale,
            PixelFormatEnum::RGBA8888,
        )?;

        // Fill surface with background color
        surf.fill_rect(None, self.bg_color)?;

        // Obtain raw surface data reference, then draw characters of string
        // through `draw_raw`
        if surf.must_lock() {
            surf.with_lock_mut(|px: &mut [u8]| self.draw_raw(px, &width, text))?
        } else {
            self.draw_raw(surf.without_lock_mut().unwrap(), &width, text)?
        }

        Ok(surf)
    }

    /// Sums the width of each character in the supplied text, and multiples the
    /// sum by the renderer's integer scale factor. Takes into consideration
    /// formatting options' effects on text width.
    pub fn measure_width(&self, text: &str) -> Result<u32, String> {
        let mut basic_width = self.scale * count_char_width(text)?;

        if self.bold {
            basic_width += 1
        }
        if self.italic {
            basic_width += 7
        }

        Ok(basic_width)
    }

    /// May in the future take into consideration newlines and other formatting.
    /// For now, it just returns `16 * scale`, thus, the result of this method
    /// can always be safely `unwrap()`ped.
    pub fn measure_height(&self, _text: &str) -> Result<u32, String> {
        Ok(self.scale * UNIFONT_HEIGHT)
    }

    /// Takes an array of pixels and draws the supplied text to it, using the
    /// specified render options. This function always assumes RGBA8888 pixel
    /// formatting.
    fn draw_raw(
        &self,
        pixels: &mut [u8],
        surf_width: &u32,
        text: &str,
    ) -> Result<(), String> {
        let unifont = get_unifont()?;

        // Start position of next character
        let mut x_offset = 0;

        let iter = text.chars();
        for c in iter {
            // Retrieve character description from hashmap
            let font_char = match unifont.get(&(c as u32)) {
                None => return Err(gen_missing_char_str(&c)),
                Some(font_char) => font_char,
            };

            // Draw rows of character bitmap
            for row in 0..UNIFONT_HEIGHT as usize {
                // Draw each pixel for a row
                for col in (0..font_char.width as usize).rev() {
                    if font_char.bitmap[row].get_bit(col) {
                        // Double character on x axis if we're bolding
                        for x in if self.bold {
                            0..self.scale * 2
                        } else {
                            0..self.scale
                        } {
                            for y in 0..self.scale {
                                // Calculate the byte position of the pixel
                                // (this thing is a mess, to be honest)
                                let px_base = (4
                                    * surf_width
                                    * (row as u32 * self.scale + y)
                                    + 4 * x_offset
                                    + 4 * (font_char.width as u32 * self.scale
                                        - col as u32 * self.scale
                                        - self.scale)
                                    + 4 * x)
                                    as usize;

                                // Insert fg colour into the current pixel
                                // TODO assumes little endian
                                pixels[px_base + 3] = self.fg_color.r;
                                pixels[px_base + 2] = self.fg_color.g;
                                pixels[px_base + 1] = self.fg_color.b;
                                pixels[px_base] = self.fg_color.a;
                            }
                        }
                    }
                }
            }

            // Shift next character
            x_offset += self.scale * font_char.width as u32;
        }

        Ok(())
    }
}

/// Maps `unifont`'s `Result` error type to ours, so that the `?` operator
/// can be utilised.
fn get_unifont<'a>() -> Result<&'a unifont::FontChars, String> {
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
fn count_char_width(text: &str) -> Result<u32, String> {
    let unifont = get_unifont()?;

    let mut width_sum: u32 = 0;
    let iter = text.chars();

    for c in iter {
        match unifont.get(&(c as u32)) {
            None => return Err(gen_missing_char_str(&c)),
            Some(fc) => width_sum += fc.width as u32,
        }
    }

    Ok(width_sum)
}

fn gen_missing_char_str(c: &char) -> String {
    format!(
        "Embedded Unifont does not contain {} (code point: 0x{:x})",
        c, *c as u32
    )
}
