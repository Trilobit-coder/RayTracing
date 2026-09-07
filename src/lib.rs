//! A path-tracing ray tracer based on *Ray Tracing in One Weekend*.
//!
//! The crate is organized into three modules:
//!
//! - [`hittable`] — scene objects a ray can intersect
//! - [`optical`] — camera, light transport, and surface materials
//! - [`utilities`] — vectors, rays, colors, intervals, and RNG helpers

#![warn(missing_docs)]

/// Scene objects and ray-intersection records.
pub mod hittable;
/// Camera, light transport, and surface materials.
pub mod optical;
/// Sample scenes.
pub mod scene;
/// Math and I/O helpers: vectors, rays, colors, intervals, RNG.
pub mod utilities;
