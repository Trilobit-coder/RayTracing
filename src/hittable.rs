use std::sync::Arc;

use crate::optical::material::Material;
use crate::utilities::Aabb;
use crate::utilities::Interval;
use crate::utilities::Ray;
use crate::utilities::{Point3, Vec3};

/// A record of the closest hit of a ray with a scene object.
#[derive(Clone)]
pub struct HitRecord {
    /// Point where the ray hit the surface.
    pub p: Point3,
    /// Outward surface normal at the hit point, oriented against the ray.
    pub normal: Vec3,
    /// Material of the hit object.
    pub mat: Option<Arc<dyn Material>>,
    /// Ray parameter `t` at the hit point.
    pub t: f32,
    /// Whether the ray hit the front face of the surface.
    pub front_face: bool,
    /// `u,v` surface coordinates of the ray-object hit point.
    pub u: f32,
    /// `u,v` surface coordinates of the ray-object hit point.
    pub v: f32,
}

/// An object a ray can hit.
pub trait Hittable: Send + Sync {
    /// Tests the ray against the object; on a hit within `ray_t`, fills `rec` and returns `true`.
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    /// Return the bounding AABB of a hittable object.
    fn bounding_box(&self) -> Aabb;
}

impl HitRecord {
    /// Sets the hit record normal, flipping it so it always opposes the ray direction.
    ///
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * -1.0
        };
    }

    /// Return an empty `HitRecord`
    pub fn empty() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat: None,
            t: 0.0,
            front_face: false,
            u: 0.0,
            v: 0.0,
        }
    }
}

/// A block built by quads.
pub mod block;
/// The bounding volume hierarchy node.
pub mod bvh;
/// A collection of [`Hittable`] objects.
pub mod hittable_list;
/// A translated or rotated instance of a hittable object.
pub mod instance;
/// A parallelogram primitive.
pub mod quad;
/// A sphere primitive.
pub mod sphere;

pub use block::Block;
pub use bvh::BvhNode;
pub use hittable_list::HittableList;
pub use instance::RotationY;
pub use instance::Translate;
pub use quad::Quad;
pub use sphere::Sphere;
