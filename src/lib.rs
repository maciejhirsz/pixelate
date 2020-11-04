// Pixelate  Copyright (C) 2019  Maciej Hirsz
//
// This file is part of Pixelate. This program comes with ABSOLUTELY NO WARRANTY;
// This is free software, and you are welcome to redistribute it under the
// conditions of the GNU General Public License version 3.0.
//
// You should have received a copy of the GNU General Public License
// along with Pixelate.  If not, see <http://www.gnu.org/licenses/>

//! ![Pixelate](https://raw.githubusercontent.com/maciejhirsz/pixelate/master/pixelate.png)
//! ## This logo is rendered with this crate
//!
//! It's only 861 bytes for a 720x128 image. [Check out the code](https://github.com/maciejhirsz/pixelate/blob/master/tests/lib.rs).
//!
//! ## Why is this even a thing?!
//!
//! Rendering up-scaled pixelated PNG files can be quite useful for things like:
//! + QR codes.
//! + Identicons like [Blockies](https://crates.io/crates/blockies).
//! + Pixel Art?
//!
//! Usually a PNG image data, before compression, is a bitmap that packs 3 or 4 bytes per pixel, depending whether transparency is present or not. For all of the cases above this is way more information than necessary.
//!
//! PNG supports an **indexed palette** format. Using the indexing palette makes it possible not only to pack a single pixel into a single byte, but for small palettes **a pixel can be 4, 2, or even just 1 _bit_**. The logo here is using 3 colors (black, transparent background and shadow), which allows Pixelate to produce a bitmap where each pixel takes only 2 bits.
//!
//! Not only does this produce smaller images after compression, smaller bitmap is also much faster to compress, as much as 10x for small palettes. This makes it ideal for rendering QR codes or identicons on the fly.

use std::io;
use std::iter::repeat;
use png::{BitDepth, ColorType};

/// Generic error type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// There has been an internal error when doing the PNG encoding
    PngEncoding,

    /// There has been an error writing the PNG to IO
    Io,

    /// The palette is over 256 colors
    PaletteTooBig,

    /// The palette is smaller than 2 colors (what's the point?)
    PaletteTooSmall,
}

impl From<png::EncodingError> for Error {
    fn from(err: png::EncodingError) -> Error {
        use png::EncodingError;

        match err {
            EncodingError::IoError(_) => Error::Io,
            EncodingError::Format(_) => Error::PngEncoding,
        }
    }
}

/// Pretty self descriptive
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    /// Red, Green, and Blue, in that order
    Rgb(u8, u8, u8),

    /// Red, Green, Blue, and Alpha, in that order
    Rgba(u8, u8, u8, u8),
}

pub const WHITE: Color = Color::Rgb(255, 255, 255);
pub const BLACK: Color = Color::Rgb(0, 0, 0);

pub struct Image<'a> {
    /// Palette of colors, up to 255 colors
    pub palette: &'a [Color],

    /// Unscaled pixels where each byte is a valid index into `palette`
    pub pixels: &'a [u8],

    /// Width of the unscaled image, `pixels` length must be divisible by `width`
    pub width: usize,

    /// Scale to render the image at
    pub scale: usize,
}

impl<'a> Image<'a> {
    /// Render the
    pub fn render<W: io::Write>(&self, writer: W) -> Result<(), Error> {
        if self.palette.len() > 256 {
            return Err(Error::PaletteTooBig);
        }

        if self.palette.len() < 2 {
            return Err(Error::PaletteTooSmall);
        }

        let bit_depth = self.bit_depth();
        let depth = bit_depth as usize;
        let pixels_in_byte = 8 / depth;

        let (img_width, img_height) = self.dimensions();
        let bytes_per_line = ceil_div(img_width, pixels_in_byte);

        let mut data = vec![0; bytes_per_line * img_height];

        let chunk_size = bytes_per_line * self.scale;

        for (chunk, pixels) in data.chunks_mut(chunk_size).zip(self.pixels.chunks(self.width)) {
            let (first_line, chunk) = chunk.split_at_mut(bytes_per_line);

            // Take the original row of pixels, repeat each pixel `scale` times
            let mut pixels = pixels.iter().flat_map(|pixel| repeat(*pixel).take(self.scale));

            // Rasterize the first row of pixels
            for byte in first_line.iter_mut() {
                // Pack as many pixels as necessary into the byte
                for (idx, pixel) in (&mut pixels).take(pixels_in_byte).enumerate() {
                    *byte |= pixel << (8 - depth) - (idx * depth);
                }
            }

            // Now repeat it until we've filled up the whole `scale`-tall section
            for row in chunk.chunks_mut(bytes_per_line) {
                row.copy_from_slice(first_line);
            }
        }

        let mut encoder = png::Encoder::new(writer, img_width as u32, img_height as u32);

        encoder.set_color(ColorType::Indexed);
        encoder.set_depth(bit_depth);

        let mut writer = encoder.write_header()?;

        writer.write_chunk(png::chunk::PLTE, &self.palette_data())?;

        if let Some(transparency) = self.transparency() {
            writer.write_chunk(png::chunk::tRNS, &transparency)?;
        }

        writer.write_image_data(&data)?;

        Ok(())
    }

    fn dimensions(&self) -> (usize, usize) {
        let width = self.width * self.scale;
        let height = (self.pixels.len() / self.width) * self.scale;

        (width, height)
    }

    fn bit_depth(&self) -> BitDepth {
        match self.palette.len() as u8 {
            0..=2  => BitDepth::One,
            3..=4  => BitDepth::Two,
            5..=16 => BitDepth::Four,
            _      => BitDepth::Eight,
        }
    }

    fn palette_data(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.palette.len() * 3);

        for color in self.palette.iter().cloned() {
            match color {
                Color::Rgb(r, g, b) | Color::Rgba(r, g, b, _) => data.extend_from_slice(&[r,g,b])
            }
        }

        data
    }

    fn transparency(&self) -> Option<Vec<u8>> {
        let mut data = None;

        let len = self.palette.len();

        for (idx, color) in self.palette.iter().enumerate() {
            let alpha = match color {
                Color::Rgb(_, _, _) => continue,
                Color::Rgba(_, _, _, alpha) => *alpha,
            };

            let mut buf = data.take().unwrap_or_else(|| vec![255; len]);

            buf[idx] = alpha;

            data = Some(buf);
        }

        data
    }
}

fn ceil_div(a: usize, b: usize) -> usize {
    a / b + (a % b != 0) as usize
}
