use crate::utilities::{Point3, Vec3, random::random_i32_range};

const POINT_COUNT: usize = 256;

/// Perlin noise generator (value noise with integer lattice).
pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    /// Creates a new Perlin noise generator with random values and permutations.
    pub fn new() -> Perlin {
        // Fill randfloat with random numbers in [0, 1)
        let mut randvec = [Vec3::zero(); POINT_COUNT];
        for v in &mut randvec {
            *v = Vec3::unit_vector(&Vec3::random_range(-1.0, 1.0));
        }

        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();

        Perlin {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    /// Returns the noise value at point `p`.
    pub fn noise(&self, p: &Point3) -> f32 {
        let u = p.x() - f32::floor(p.x());
        let v = p.y() - f32::floor(p.y());
        let w = p.z() - f32::floor(p.z());

        let i = f32::floor(p.x()) as i32;
        let j = f32::floor(p.y()) as i32;
        let k = f32::floor(p.z()) as i32;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx_x = ((i + di) & 255) as usize;
                    let idx_y = ((j + dj) & 255) as usize;
                    let idx_z = ((k + dk) & 255) as usize;
                    let perm_idx = self.perm_x[idx_x] ^ self.perm_y[idx_y] ^ self.perm_z[idx_z];
                    c[di as usize][dj as usize][dk as usize] = self.randvec[perm_idx];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    /// Returns the absolute value of summed octaves of Perlin noise at increasing frequencies (turbulence).
    pub fn turb(&self, p: &Point3, depth: usize) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        f32::abs(accum)
    }

    fn generate_perm() -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for (i, val) in p.iter_mut().enumerate() {
            *val = i;
        }
        Self::permute(&mut p);
        p
    }

    fn permute(p: &mut [usize; POINT_COUNT]) {
        for i in (1..p.len()).rev() {
            let target = random_i32_range(0, i as i32) as usize;
            p.swap(i, target);
        }
    }

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for (i, row) in c.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                for (k, &vec) in col.iter().enumerate() {
                    let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    let wi = i as f32 * uu + (1.0 - i as f32) * (1.0 - uu);
                    let wj = j as f32 * vv + (1.0 - j as f32) * (1.0 - vv);
                    let wk = k as f32 * ww + (1.0 - k as f32) * (1.0 - ww);
                    accum += wi * wj * wk * Vec3::dot(&vec, &weight_v);
                }
            }
        }
        accum
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
