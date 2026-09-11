use std::{
    f32::{self},
    sync::Arc,
};

use crate::{
    hittable::{HitRecord, Hittable},
    utilities::{Aabb, Interval, Point3, Ray, Vec3},
};

/// A tranlated instance of a hittable primitive.
pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    /// create a translated instance of object along an offset.
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Translate {
        Translate {
            object: object.clone(),
            offset,
            bbox: object.bounding_box() + offset,
        }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn has_motion(&self) -> bool {
        self.object.has_motion()
    }

    fn hit<'a>(&'a self, r: &Ray, ray_t: Interval, rec: &mut HitRecord<'a>) -> bool {
        // Move the ray backwards by the offset
        let offset_r = Ray::new(r.origin() - self.offset, r.direction()).set_time(r.time());

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        // Move the intersection point forwards by the offset
        rec.p += self.offset;

        true
    }
}

/// A rotated instance of a hittable primitive about Y axis.
pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Aabb,
}

impl RotateY {
    /// create a rotated object about Y by an angle in degrees
    pub fn new(object: Arc<dyn Hittable>, angle: f32) -> RotateY {
        let radians = f32::to_radians(angle);
        let sin_theta = f32::sin(radians);
        let cos_theta = f32::cos(radians);
        let bbox = object.bounding_box();

        let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f32;
                    let j = j as f32;
                    let k = k as f32;

                    let x = i * bbox.x.max + (1. - i) * bbox.x.min;
                    let y = j * bbox.y.max + (1. - j) * bbox.y.min;
                    let z = k * bbox.z.max + (1. - k) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = f32::min(min[c], tester[c]);
                        max[c] = f32::max(max[c], tester[c]);
                    }
                }
            }
        }

        let bbox = Aabb::extrema(&min, &max);

        RotateY {
            object,
            bbox,
            sin_theta,
            cos_theta,
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn has_motion(&self) -> bool {
        self.object.has_motion()
    }

    fn hit<'a>(&'a self, r: &Ray, ray_t: Interval, rec: &mut HitRecord<'a>) -> bool {
        // Transform the ray from world space to object space.

        let origin = Point3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()),
            r.origin().y(),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z()),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new(origin, direction).set_time(r.time());

        // Determine whether an intersection exists in object space (and if so, where).

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        // Transform the intersection from object space back to world space.
        rec.p = Point3::new(
            (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.z()),
            rec.p.y(),
            (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.z()),
        );

        rec.normal = Vec3::new(
            (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()),
            rec.normal.y(),
            (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z()),
        );

        true
    }
}
