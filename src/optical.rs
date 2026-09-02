/// The camera model and scene rendering.
pub mod camera;
/// Surface materials that scatter rays.
pub mod material;
/// Light transport: ray colors, reflection, refraction, reflectance.
pub mod optics;

pub use camera::Camera;
