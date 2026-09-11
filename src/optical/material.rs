use crate::{
    hittable::HitRecord,
    optical::{
        optics::{reflect, reflectance, refract},
        texture::Texture,
    },
    utilities::{Color, Point3, Ray, Vec3, random::random_f32},
};

/// A material determines how rays scatter when they hit a surface.
///
/// An enum instead of a trait object so scattering is a match instead of a
/// vtable call, and materials are cheap to clone.
#[derive(Clone)]
pub enum Material {
    /// A diffuse material.
    Lambertian(Texture),
    /// A reflective metal.
    Metal(Metal),
    /// A dielectric (glass-like) material.
    Dielectric(Dielectric),
    /// A light emitting material.
    DiffuseLight(Texture),
    /// An isotropic scattering material.
    Isotropic(Texture),
}

impl Material {
    /// Creates a diffuse material from a texture (or a solid [`Color`]).
    pub fn lambertian(tex: impl Into<Texture>) -> Material {
        Material::Lambertian(tex.into())
    }

    /// Creates a reflective metal with the given albedo and fuzz.
    pub const fn metal(albedo: Color, fuzz: f32) -> Material {
        Material::Metal(Metal::new(albedo, fuzz))
    }

    /// Creates a dielectric with the given refractive index.
    pub const fn dielectric(refraction_index: f32) -> Material {
        Material::Dielectric(Dielectric::new(refraction_index))
    }

    /// Creates a light emitting material from a texture (or a solid [`Color`]).
    pub fn diffuse_light(tex: impl Into<Texture>) -> Material {
        Material::DiffuseLight(tex.into())
    }

    /// Creates an isotropic material from a texture (or a solid [`Color`]).
    pub fn isotropic(tex: impl Into<Texture>) -> Material {
        Material::Isotropic(tex.into())
    }

    /// Scatters an incoming ray at a hit point, returning the scattered ray and
    /// its attenuation, or `None` if the ray is absorbed.
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian(tex) => scatter_lambertian(tex, r_in, rec),
            Material::Metal(metal) => metal.scatter(r_in, rec),
            Material::Dielectric(dielectric) => dielectric.scatter(r_in, rec),
            Material::DiffuseLight(_) => None,
            Material::Isotropic(tex) => scatter_isotropic(tex, r_in, rec),
        }
    }

    /// Emits the light color of the material at the hit point; completely non-emitting by default.
    pub fn emitted(&self, u: f32, v: f32, p: &Point3) -> Color {
        match self {
            Material::DiffuseLight(tex) => tex.value(u, v, p),
            _ => Color::new(0., 0., 0.),
        }
    }
}

/// Diffuse scattering: bounce in a random unit sphere direction above the surface.
fn scatter_lambertian(tex: &Texture, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

    // Catch degenerate scatter direction
    if scatter_direction.near_zero() {
        scatter_direction = rec.normal;
    }

    let scattered = Ray::new(rec.p, scatter_direction).set_time(r_in.time());
    let attenuation = tex.value(rec.u, rec.v, &rec.p);

    Some((attenuation, scattered))
}

/// Isotropic scattering: bounce in a uniformly random direction.
fn scatter_isotropic(tex: &Texture, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    let scattered = Ray::new(rec.p, Vec3::random_unit_vector()).set_time(r_in.time());
    let attenuation = tex.value(rec.u, rec.v, &rec.p);

    Some((attenuation, scattered))
}

/// A reflective metal with a fuzz factor.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    /// Creates a metal material with the given albedo and fuzz.
    pub const fn new(albedo: Color, fuzz: f32) -> Metal {
        let fuzz = fuzz.clamp(0.0, 1.0);

        Metal { albedo, fuzz }
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&r_in.direction(), &rec.normal);
        let reflected = Vec3::unit_vector(&reflected) + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(rec.p, reflected).set_time(r_in.time());
        let attenuation = self.albedo;

        if Vec3::dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

/// A dielectric (glass-like) material.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f32,
}

impl Dielectric {
    /// Creates a dielectric material with the given refractive index.
    pub const fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(&r_in.direction());
        let cos_theta = f32::min(Vec3::dot(&(unit_direction * -1.0), &rec.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, ri) > random_f32() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, ri)
        };

        let scattered = Ray::new(rec.p, direction).set_time(r_in.time());
        Some((attenuation, scattered))
    }
}
