use crate::utilities::{Point3, Vec3};

/// A ray: P(t) = origin + t * direction.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f32,
}

impl Ray {
    /// Creates a ray from an origin and direction.
    pub const fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            time: 0.0,
        }
    }

    /// Set the time information of a ray
    pub const fn set_time(mut self, time: f32) -> Self {
        self.time = time;

        self
    }

    /// Returns the point on the ray at parameter `t`: `P = origin + t * direction`.
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }

    /// The ray origin.
    pub const fn origin(&self) -> Point3 {
        self.orig
    }
    /// The ray direction.
    pub const fn direction(&self) -> Vec3 {
        self.dir
    }
    /// The ray time.
    pub const fn time(&self) -> f32 {
        self.time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_at() {
        let origin = Point3::zero();
        let direction = Vec3::new(0.3, 0.4, 0.5);
        let ray = Ray::new(origin, direction);

        let result_at_ten = ray.at(10.0);
        let expect = Point3::new(3.0, 4.0, 5.0);

        assert_eq!(expect, result_at_ten);
    }
}
