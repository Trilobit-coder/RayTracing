use std::{path::Path, sync::Arc};

use crate::utilities::{Color, Image, Point3, perlin::Perlin};

/// A texture maps surface coordinates to a color.
pub trait Texture: Send + Sync {
    /// Samples the texture at texture coordinates `(u, v)` and surface point `p`.
    fn value(&self, u: f32, v: f32, p: Point3) -> Color;
}

/// A trait for mapping a 3D surface point to 2D texture coordinates (u, v).
/// Typically used to feed into a [`Texture`] for sampling.
///
/// The coordinates are usually expected to be in the range [0.0, 1.0], but
/// implementations may return any range as long as the consuming texture
/// handles it correctly (e.g., with wrap or clamp modes).
pub trait UVMap {
    /// Returns the (u, v) texture coordinates for the given surface point `p`.
    fn uv(p: &Point3) -> (f32, f32);
}

/// A texture that returns the same color everywhere.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    /// Creates a solid-color texture with the given albedo.
    pub const fn new(albedo: Color) -> SolidColor {
        SolidColor { albedo }
    }

    /// Creates a solid-color texture from RGB components.
    pub const fn from_rgb(red: f32, green: f32, blue: f32) -> SolidColor {
        SolidColor {
            albedo: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: Point3) -> Color {
        self.albedo
    }
}

/// A procedural checkerboard that alternates between two sub-textures.
#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f32,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    /// Creates a checker texture alternating between `even` and `odd`, with cells of the given scale.
    pub const fn new(scale: f32, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    /// Creates a checker texture from two solid colors, with cells of the given scale.
    pub fn from_colors(scale: f32, c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(c1)) as Arc<dyn Texture>,
            odd: Arc::new(SolidColor::new(c2)) as Arc<dyn Texture>,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, _u: f32, _v: f32, p: Point3) -> Color {
        let x_int = f32::floor(self.inv_scale * p.x()) as i32;
        let y_int = f32::floor(self.inv_scale * p.y()) as i32;
        let z_int = f32::floor(self.inv_scale * p.z()) as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even.value(_u, _v, p)
        } else {
            self.odd.value(_u, _v, p)
        }
    }
}

/// A texture maps the surface to a image.
pub struct ImageTexture {
    image: Image,
}

impl ImageTexture {
    /// Create a imagetexture from loading a path.
    /// Will become a solid magenta if image does not exist.
    pub fn new(filename: impl AsRef<Path>) -> ImageTexture {
        ImageTexture {
            image: Image::load(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: Point3) -> Color {
        self.image.pixel_data(u, v)
    }
}

/// A texture that samples Perlin noise to produce random values.
pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    /// Creates a new `NoiseTexture` with a freshly generated Perlin noise table.
    pub fn new(scale: f32) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Point3) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + f32::sin(self.scale * p.z() + 10. * self.noise.turb(p, 7)))
    }
}
