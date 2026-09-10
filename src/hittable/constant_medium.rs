use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    optical::{
        Texture,
        material::{Isotropic, Material},
    },
    utilities::{Color, Interval, Ray, Vec3, random::random_f32},
};

/// A participating medium of uniform density filling the volume of `boundary`.
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    /// Creates a medium of constant `density` filling `boundary`, scattering with `tex`.
    pub fn new(boundary: Arc<dyn Hittable>, density: f32, tex: Arc<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(tex)),
        }
    }

    /// Creates a medium of constant `density` filling `boundary`, scattering with `SolidTexture` with given albedo.
    pub fn from_color(boundary: Arc<dyn Hittable>, density: f32, albedo: Color) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self) -> crate::utilities::Aabb {
        self.boundary.bounding_box()
    }

    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::empty();
        let mut rec2 = HitRecord::empty();

        if !self.boundary.hit(r, Interval::universe(), &mut rec1) {
            return false;
        }

        if !self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, f32::INFINITY), &mut rec2)
        {
            return false;
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0. {
            rec1.t = 0.;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f32::ln(random_f32());

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1., 0., 0.); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.mat = Some(self.phase_function.clone());

        true
    }
}
