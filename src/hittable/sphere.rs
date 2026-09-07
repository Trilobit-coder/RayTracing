use std::f32::consts::PI;
use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::optical::material::Material;
use crate::optical::texture::UVMap;
use crate::utilities::{Aabb, Interval, Ray};
use crate::utilities::{Point3, Vec3};

/// A sphere, defined by its center, radius, and material.
#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f32,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    /// Creates a stationary sphere; negative radii are clamped to zero.
    pub fn new(center: Point3, radius: f32, mat: Arc<dyn Material>) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::extrema(&(center - rvec), &(center + rvec));
        Self {
            center: Ray::new(center, Vec3::zero()),
            radius: radius.max(0.0),
            mat,
            bbox,
        }
    }

    /// Enable a moving sphere by passing the center at `time=1`.
    pub fn motion(mut self, to: Point3) -> Self {
        let rvec = Vec3::new(self.radius, self.radius, self.radius);
        let to_box = Aabb::extrema(&(to - rvec), &(to + rvec));
        self.center = Ray::new(self.center.origin(), to - self.center.origin());
        self.bbox = Aabb::merge(&self.bbox, &to_box);

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
        (rec.u, rec.v) = Sphere::uv(&outward_normal);
        rec.mat = Some(self.mat.clone());

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

impl UVMap for Sphere {
    fn uv(p: &Point3) -> (f32, f32) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = f32::acos(-p.y());
        let phi = f32::atan2(-p.z(), p.x()) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}
