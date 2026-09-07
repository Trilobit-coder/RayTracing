use crate::{
    hittable::{HitRecord, Hittable},
    utilities::{Color, Interval, Ray, Vec3},
};

/// Returns the color seen along `r` as it bounces through `world`, up to `depth` bounces.
pub fn ray_color<T: Hittable>(r: &Ray, depth: u32, world: &T) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::zero();
    }

    let mut rec = HitRecord::empty();

    if world.hit(r, Interval::new(0.001, f32::INFINITY), &mut rec) {
        if let Some(mat) = rec.mat.as_ref()
            && let Some((attenuation, scattered)) = mat.scatter(r, &rec)
        {
            return attenuation * ray_color(&scattered, depth - 1, world);
        }
        return Color::zero();
    }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

/// Reflects vector `v` about the normal `n`.
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * *n
}

/// Refracts unit vector `uv` through a surface with normal `n`,
/// where `etai_over_etat` is the ratio of the two refractive indices.
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(Vec3::dot(&(*uv * -1.0), n), 1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * *n;

    r_out_perp + r_out_parallel
}

/// Schlick's approximation of the reflectance for the cosine `cosine`
/// and refractive index ratio `refractance_index`.
pub fn reflectance(cosine: f32, refractance_index: f32) -> f32 {
    let r0 = (1.0 - refractance_index) / (1.0 + refractance_index);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
}
