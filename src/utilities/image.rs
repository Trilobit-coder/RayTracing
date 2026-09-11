use std::path::Path;

use image::DynamicImage;

use crate::utilities::Color;

/// A decoded RGB image, sampled with continuous `(u, v)` coordinates.
pub struct Image {
    width: u32,
    height: u32,
    /// Bottom-up linear-light RGB pixel data, `3 * width * height` f32s;
    /// `None` if the image could not be loaded.
    bdata: Option<Vec<f32>>,
}

impl Image {
    /// Directories searched, in order, for `filename` when it does not name an
    /// existing file. The empty prefix means "try the filename as given".
    pub const SEARCH_DIRS: [&'static str; 2] = ["assets", ""];

    /// Loads an image from `filename` in any format the [`image`] crate supports.
    ///
    /// Searches [`Self::SEARCH_DIRS`] for the first candidate that exists and
    /// decodes. On failure prints an error and returns an image that samples
    /// as magenta
    pub fn load(filename: impl AsRef<Path>) -> Image {
        let filename = filename.as_ref();
        let candidates = Self::SEARCH_DIRS
            .iter()
            .map(|dir| Path::new(dir).join(filename));

        for candidate in candidates {
            if !candidate.exists() {
                continue;
            }
            match image::open(&candidate) {
                Ok(img) => return Image::from_dynamic(img),
                Err(e) => {
                    eprintln!(
                        "WARNING: Could not load image file '{}': {e}",
                        candidate.display()
                    );
                }
            }
        }

        eprintln!(
            "ERROR: Could not find image file '{:?}' in any search path",
            filename
        );
        Image::empty()
    }

    /// Creates an empty image that samples as magenta; used when loading fails.
    fn empty() -> Image {
        Image {
            width: 0,
            height: 0,
            bdata: None,
        }
    }

    fn from_dynamic(img: DynamicImage) -> Image {
        let img = img.to_rgb8();
        let (width, height) = img.dimensions();
        let mut bdata = vec![0.0; 3 * width as usize * height as usize];

        // Decoded rows run top-down, but the texture convention wants v = 0 at
        // the bottom of the image, so flip vertically while copying. Stored
        // pixels are gamma-encoded (sRGB), but the renderer works in linear
        // light; undo the encoding once here with the same gamma 2
        // approximation that `write_color` applies on output, so the texture
        // round-trips to its original appearance.
        for (x, y, px) in img.enumerate_pixels() {
            let dst = 3 * ((height - 1 - y) * width + x) as usize;
            bdata[dst] = (px.0[0] as f32 / 255.0).powi(2);
            bdata[dst + 1] = (px.0[1] as f32 / 255.0).powi(2);
            bdata[dst + 2] = (px.0[2] as f32 / 255.0).powi(2);
        }

        Image {
            width,
            height,
            bdata: Some(bdata),
        }
    }

    /// The image width in pixels.
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The image height in pixels.
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Samples the image at continuous texture coordinates `(u, v)`, clamped
    /// to the image bounds. Returns magenta if the image failed to load.
    pub fn pixel_data(&self, u: f32, v: f32) -> Color {
        const MAGENTA: Color = Color::new(1.0, 0.0, 1.0);
        let Some(bdata) = &self.bdata else {
            return MAGENTA;
        };

        let i = (u * self.width as f32) as usize;
        let j = (v * self.height as f32) as usize;
        let i = i.clamp(0, self.width as usize - 1);
        let j = j.clamp(0, self.height as usize - 1);

        let idx = 3 * (i + self.width as usize * j);
        Color::new(bdata[idx], bdata[idx + 1], bdata[idx + 2])
    }
}
