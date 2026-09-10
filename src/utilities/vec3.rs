use std::{fmt, ops};

use crate::utilities::random::{random_f32, random_f32_range};

/// A 3D point in space.
pub type Point3 = Vec3;

/// A 3D vector with `f32` components.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    /// Creates a vector from its three components.
    pub const fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    /// Returns the zero vector.
    pub const fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    /// Returns the length of the vector.
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    /// Returns the squared length of the vector.
    pub const fn length_squared(&self) -> f32 {
        Vec3::dot(self, self)
    }

    /// Returns the dot product of two vectors.
    pub const fn dot(u: &Vec3, v: &Vec3) -> f32 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    /// Returns the cross product of two vectors.
    pub const fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0],
            ],
        }
    }

    /// Returns the vector scaled to unit length.
    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }

    /// Returns a vector with random components in `[0, 1)`.
    pub fn random() -> Vec3 {
        Vec3::new(random_f32(), random_f32(), random_f32())
    }

    /// Returns a vector with random components in `[min, max)`.
    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            random_f32_range(min, max),
            random_f32_range(min, max),
            random_f32_range(min, max),
        )
    }

    /// Returns a random vector of unit length.
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-22 < lensq && lensq <= 1.0 {
                return p / f32::sqrt(lensq);
            }
        }
    }

    /// Returns a random vector inside the unit disk (`z = 0`).
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_f32_range(-1.0, 1.0),
                random_f32_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Returns `true` if the vector is close to zero in all dimensions.
    pub const fn near_zero(&self) -> bool {
        let s = 1e-6;
        (f32::abs(self.e[0]) < s) && (f32::abs(self.e[1]) < s) && (f32::abs(self.e[2]) < s)
    }

    /// Returns a random unit vector in the same hemisphere as `normal`.
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_shpere = Vec3::random_unit_vector();
        if Vec3::dot(&on_unit_shpere, normal) > 0.0
        // In the same hemishpere as the normal
        {
            on_unit_shpere
        } else {
            on_unit_shpere * -1.0
        }
    }

    /// Returns the `x` component.
    pub const fn x(&self) -> f32 {
        self.e[0]
    }
    /// Returns the `y` component.
    pub const fn y(&self) -> f32 {
        self.e[1]
    }
    /// Returns the `z` component.
    pub const fn z(&self) -> f32 {
        self.e[2]
    }

    /// Returns the red component.
    pub const fn r(&self) -> f32 {
        self.e[0]
    }
    /// Returns the green component.
    pub const fn g(&self) -> f32 {
        self.e[1]
    }
    /// Returns the blue component.
    pub const fn b(&self) -> f32 {
        self.e[2]
    }
}

// Vec3 + Vec3
impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

// Vec3 += Vec3
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

// Vec3 - Vec3
impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

// Vec3 -= Vec3
impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

// Vec3 * Vec3
impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

// Vec3 * f32
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

// f32 * Vec3
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}

// Vec3 *= f32
impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

// Vec3 / f32
impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

// Vec3 /= f32
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

// -Vec3
impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

// display: [e0, e1, e2]
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.e[0], self.e[1], self.e[2])
    }
}

// indexing
impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, idx: usize) -> &f32 {
        &self.e[idx]
    }
}
impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut f32 {
        &mut self.e[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(9.0, 5.0, 4.0);

        let expect = Vec3::new(10.0, 7.0, 7.0);
        assert_eq!(expect, v1 + v2);

        v1 += v2;
        assert_eq!(expect, v1);
    }

    #[test]
    fn sub() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(9.0, 5.0, 4.0);

        let expect = Vec3::new(-8.0, -3.0, -1.0);
        assert_eq!(expect, v1 - v2);

        v1 -= v2;
        assert_eq!(expect, v1);
    }

    #[test]
    fn mul() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(9.0, 5.0, 4.0);

        let expect = Vec3::new(9.0, 10.0, 12.0);
        assert_eq!(expect, v1 * v2);

        let scalar = 3.0;

        let expect = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(expect, v1 * scalar);
        assert_eq!(expect, scalar * v1);

        v1 *= scalar;
        assert_eq!(expect, v1);
    }

    #[test]
    fn div() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 2.0;

        let expect = Vec3::new(0.5, 1.0, 1.5);
        assert_eq!(expect, v1 / scalar);

        v1 /= scalar;
        assert_eq!(expect, v1);
    }

    #[test]
    fn length() {
        let v1 = Vec3::new(1.0, 2.0, 2.0);

        let expect = 3.0;
        assert_eq!(expect, v1.length());
    }

    #[test]
    fn dot_and_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(9.0, 5.0, 4.0);

        let result = Vec3::dot(&v1, &Vec3::cross(&v1, &v2));
        let expect = 0.0;

        assert_eq!(expect, result);

        assert_eq!(Vec3::zero(), Vec3::cross(&v1, &v1))
    }
}
