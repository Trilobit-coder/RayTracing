use crate::utilities::{Interval, Vec3};
use std::io::{Result, Write};

/// An RGB color.
pub type Color = Vec3;

const INTENSITY: Interval = Interval::new(0.000, 0.999);

/// Writes a color as a PPM pixel line, applying gamma correction.
pub fn write_color<W: Write>(out: &mut W, color: &Color) -> Result<()> {
    // Apply a linear to gamma transform for gamma 2
    let r = linear_to_gamma(color.r());
    let g = linear_to_gamma(color.g());
    let b = linear_to_gamma(color.b());

    // Translate the [0,1] component values to the byte range [0,255].
    let ir = (256.0 * INTENSITY.clamp(r)) as u32;
    let ig = (256.0 * INTENSITY.clamp(g)) as u32;
    let ib = (256.0 * INTENSITY.clamp(b)) as u32;

    // Write out the pixel color components.
    writeln!(out, "{} {} {}", ir, ig, ib)
}

fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        return f32::sqrt(linear_component);
    }

    0.0
}
