use std::sync::Arc;

use crate::{
    hittable::HitRecord,
    optical::{
        Texture,
        optics::{reflect, reflectance, refract},
        texture::SolidColor,
    },
    utilities::{Color, Point3, Ray, Vec3, random::random_f32},
};

/// A material determines how rays scatter when they hit a surface.
pub trait Material: Send + Sync {
    /// Scatters an incoming ray at a hit point, returning the scattered ray and
    /// its attenuation, or `None` if the ray is absorbed.
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    /// emitting a color (light), will be completely non-emitting as default.
    fn emitted(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        Color::new(0., 0., 0.)
    }
}

/// A diffuse material with a constant albedo.
#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    /// Creates a Lambertian material with the given albedo.
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            tex: Arc::new(SolidColor::new(albedo)) as Arc<dyn Texture>,
        }
    }
    /// Creates a Lambertian material with the given texture.
    pub const fn from_texture(tex: Arc<dyn Texture>) -> Lambertian {
        Lambertian { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction).set_time(r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);

        Some((attenuation, scattered))
    }
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
}

impl Material for Metal {
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
    /// Creates a dielectric material with the given refraction index.
    pub const fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
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

/// A light emitting material.
pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    /// create a diffuse light material from texture.
    pub fn new(tex: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { tex }
    }

    /// craete a diffuse light material from color, will behave in the same way as solid color material.
    pub fn from_color(emit: Color) -> DiffuseLight {
        DiffuseLight {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}

/// An isotropic material.
pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    /// create an isotropic material from texture.
    pub fn new(tex: Arc<dyn Texture>) -> Isotropic {
        Isotropic { tex }
    }

    /// create an isotropic material from color (the same as SolidColor texture).
    pub fn from_color(albedo: Color) -> Isotropic {
        Isotropic {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(rec.p, Vec3::random_unit_vector()).set_time(r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);

        Some((attenuation, scattered))
    }
}
