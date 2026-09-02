use crate::hittable::{HitRecord, Hittable};
use crate::utilities::{Interval, Ray};

/// A list of hittable objects; hit-testing returns the closest hit among them.
#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    /// Creates an empty list.
    pub const fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Creates a list containing a single object.
    pub fn from(object: Box<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    /// Removes all objects from the list.
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// Adds an object to the list.
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
