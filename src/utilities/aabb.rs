use std::{cmp::Ordering, ops, sync::Arc};

use crate::{
    hittable::Hittable,
    utilities::{Interval, Point3, Ray, Vec3},
};

/// An axis-aligned bounding box with intervals along all axises.
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Aabb {
    /// side along x axis.
    pub x: Interval,
    /// side along y axis.
    pub y: Interval,
    /// side along z axis.
    pub z: Interval,
}

impl Aabb {
    /// Create an AABB from three axis intervals.
    pub const fn new(x: &Interval, y: &Interval, z: &Interval) -> Aabb {
        Aabb {
            x: *x,
            y: *y,
            z: *z,
        }
        .pad_to_minimums()
    }

    /// An empty AABB.
    pub const fn empty() -> Aabb {
        Aabb {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    /// An universe AABB.
    pub const fn universe() -> Aabb {
        Aabb {
            x: Interval::universe(),
            y: Interval::universe(),
            z: Interval::universe(),
        }
    }

    /// Create an AABB from two points as extrema for the bounding box.
    pub const fn extrema(a: &Point3, b: &Point3) -> Aabb {
        // particular minimum/maximum coordinate order.
        Aabb {
            x: Interval::new(a.x().min(b.x()), a.x().max(b.x())),
            y: Interval::new(a.y().min(b.y()), a.y().max(b.y())),
            z: Interval::new(a.z().min(b.z()), a.z().max(b.z())),
        }
        .pad_to_minimums()
    }

    /// Create an AABB tightly enclosing the two input AABBs.
    pub const fn merge(box0: &Aabb, box1: &Aabb) -> Aabb {
        Aabb {
            x: Interval::merge(&box0.x, &box1.x),
            y: Interval::merge(&box0.y, &box1.y),
            z: Interval::merge(&box0.z, &box1.z),
        }
    }

    /// index based getter for return interval along an axis.
    pub const fn axis_interval(&self, n: usize) -> &Interval {
        if n == 1 {
            return &self.y;
        }
        if n == 2 {
            return &self.z;
        }
        &self.x
    }

    /// check whether a ray hit AABB under a time interval.
    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();
        let mut ray_t = *ray_t;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            let t_entry = t0.min(t1);
            let t_exit = t0.max(t1);

            ray_t.min = ray_t.min.max(t_entry);
            ray_t.max = ray_t.max.min(t_exit);

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }

    /// Compare AABBs along a given axis.
    pub fn compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        let a_bbox = a.bounding_box();
        let b_bbox = b.bounding_box();
        a_bbox
            .axis_interval(axis_index)
            .min
            .partial_cmp(&b_bbox.axis_interval(axis_index).min)
            .unwrap_or(Ordering::Equal)
    }
    /// Compare AABBs along X axis.
    pub fn x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::compare(a, b, 0)
    }
    /// Compare AABBs along Y axis.
    pub fn y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::compare(a, b, 1)
    }
    /// Compare AABBs along Z axis.
    pub fn z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::compare(a, b, 2)
    }

    /// The center point of the bounding box.
    pub fn centroid(&self) -> Point3 {
        Point3::new(
            0.5 * (self.x.min + self.x.max),
            0.5 * (self.y.min + self.y.max),
            0.5 * (self.z.min + self.z.max),
        )
    }

    /// Returns the index of the longest axis of the bounding box.
    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }

    const fn pad_to_minimums(mut self) -> Self {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }

        self
    }
}

// Aabb + Vec3
impl ops::Add<Vec3> for Aabb {
    type Output = Self;
    fn add(self, offset: Vec3) -> Aabb {
        Aabb {
            x: self.x + offset.x(),
            y: self.y + offset.y(),
            z: self.z + offset.z(),
        }
    }
}

// Vec3 + Aabb
impl ops::Add<Aabb> for Vec3 {
    type Output = Aabb;
    fn add(self, bbox: Aabb) -> Aabb {
        Aabb {
            x: self.x() + bbox.x,
            y: self.y() + bbox.y,
            z: self.z() + bbox.z,
        }
    }
}
