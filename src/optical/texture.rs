use std::{path::Path, sync::Arc};

use crate::utilities::{Color, Image, Point3, perlin::Perlin};

/// A texture maps surface coordinates to a color.
///
/// An enum instead of a trait object so sampling a texture is a match
/// instead of a vtable call, and cloning is a cheap `Arc` bump.
#[derive(Clone)]
pub enum Texture {
    /// A texture that returns the same color everywhere.
    Solid(SolidColor),
    /// A procedural checkerboard that alternates between two sub-textures.
    Checker(CheckerTexture),
    /// A texture that samples Perlin noise to produce random values.
    Noise(Arc<NoiseTexture>),
    /// A texture that maps an image onto the surface.
    Image(Arc<ImageTexture>),
}

impl Texture {
    /// Samples the texture at texture coordinates `(u, v)` and surface point `p`.
    pub fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        match self {
            Texture::Solid(solid) => solid.value(),
            Texture::Checker(checker) => checker.value(u, v, p),
            Texture::Noise(noise) => noise.value(u, v, p),
            Texture::Image(image) => image.value(u, v, p),
        }
    }

    /// Creates a solid-color texture with the given albedo.
    pub fn solid(albedo: Color) -> Texture {
        Texture::Solid(SolidColor::new(albedo))
    }

    /// Creates a checker texture alternating between two solid colors, with cells of the given scale.
    pub fn checker(scale: f32, even: Color, odd: Color) -> Texture {
        Texture::Checker(CheckerTexture::from_colors(scale, even, odd))
    }

    /// Creates a texture that samples Perlin noise, scaled by `scale`.
    pub fn noise(scale: f32) -> Texture {
        Texture::Noise(Arc::new(NoiseTexture::new(scale)))
    }

    /// Creates an image texture by loading the image from `filename`.
    /// Becomes solid magenta if the image does not exist.
    pub fn image(filename: impl AsRef<Path>) -> Texture {
        Texture::Image(Arc::new(ImageTexture::new(filename)))
    }
}

impl From<Color> for Texture {
    fn from(albedo: Color) -> Texture {
        Texture::solid(albedo)
    }
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

    /// Returns the constant albedo color.
    pub const fn value(&self) -> Color {
        self.albedo
    }
}

/// A procedural checkerboard that alternates between two sub-textures.
#[derive(Clone)]
pub struct CheckerTexture {
    inv_scale: f32,
    even: Arc<Texture>,
    odd: Arc<Texture>,
}

impl CheckerTexture {
    /// Creates a checker texture alternating between `even` and `odd`, with cells of the given scale.
    pub const fn new(scale: f32, even: Arc<Texture>, odd: Arc<Texture>) -> CheckerTexture {
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
            even: Arc::new(Texture::solid(c1)),
            odd: Arc::new(Texture::solid(c2)),
        }
    }

    /// Samples the checkerboard at surface point `p`, forwarding `(u, v)` to the chosen sub-texture.
    pub fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let x_int = f32::floor(self.inv_scale * p.x()) as i32;
        let y_int = f32::floor(self.inv_scale * p.y()) as i32;
        let z_int = f32::floor(self.inv_scale * p.z()) as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

/// A texture maps the surface to a image.
pub struct ImageTexture {
    image: Arc<Image>,
}

impl ImageTexture {
    /// Create a imagetexture from loading a path.
    /// Will become a solid magenta if image does not exist.
    pub fn new(filename: impl AsRef<Path>) -> ImageTexture {
        ImageTexture {
            image: Arc::new(Image::load(filename)),
        }
    }

    /// Samples the image at texture coordinates `(u, v)`.
    pub fn value(&self, u: f32, v: f32, _p: &Point3) -> Color {
        self.image.pixel_data(u, v)
    }
}

/// A texture that samples Perlin noise to produce random values.
pub struct NoiseTexture {
    noise: Arc<Perlin>,
    scale: f32,
}

impl NoiseTexture {
    /// Creates a new `NoiseTexture` with a freshly generated Perlin noise table.
    pub fn new(scale: f32) -> NoiseTexture {
        NoiseTexture {
            noise: Arc::new(Perlin::new()),
            scale,
        }
    }

    /// Samples turbulence-scaled noise at surface point `p`.
    pub fn value(&self, _u: f32, _v: f32, p: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + f32::sin(self.scale * p.z() + 10. * self.noise.turb(p, 7)))
    }
}
