use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::optical::material::Material;
use crate::utilities::{Interval, Ray};
use crate::utilities::{Point3, Vec3};

/// A sphere, defined by its center, radius, and material.
#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f32,
    mat: Arc<dyn Material>,
}

impl Sphere {
    /// Creates a stationary sphere; negative radii are clamped to zero.
    pub const fn new(center: Point3, radius: f32, mat: Arc<dyn Material>) -> Sphere {
        Self {
            center: Ray::new(center, Vec3::zero()),
            radius: radius.max(0.0),
            mat,
        }
    }

    /// Enable a moving sphere by passing the center at `time=1`
    pub fn motion(mut self, to: Point3) -> Self {
        self.center = Ray::new(self.center.origin(), to - self.center.origin());

        self
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f32::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surround(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surround(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());

        true
    }
}
