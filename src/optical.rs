/// The camera model and scene rendering.
pub mod camera;
/// Surface materials that scatter rays.
pub mod material;
/// Light transport: ray colors, reflection, refraction, reflectance.
pub mod optics;
/// Texture mapping model
pub mod texture;

pub use camera::Camera;
pub use texture::Texture;
