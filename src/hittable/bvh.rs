use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable, HittableList},
    utilities::{Aabb, Interval, Ray},
};

/// A node in a BVH tree, holding left/right subtrees and a bounding box.
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    /// Builds a BVH node from a mutable slice of objects.
    ///
    /// Recursively partitions the objects, sorting the slice by a random axis (x, y, or z)
    /// for slices with more than 2 objects. Returns a node with `left` and `right` children
    /// as `Arc<dyn Hittable>`, and computes the bounding box.
    ///
    /// # Panics
    /// - If the slice is empty (requires at least one object).
    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> BvhNode {
        assert!(!objects.is_empty(), "BVH requires at least one object");

        // Build the bounding box of the span of source objects.
        let mut bbox = Aabb::empty();
        for object in objects.iter() {
            bbox = Aabb::merge(&bbox, &object.bounding_box());
        }

        let axis = bbox.longest_axis();
        let comparators = [Aabb::x_compare, Aabb::y_compare, Aabb::z_compare];
        let comparator = comparators[axis];

        let len = objects.len();
        let (left, right) = match len {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => (objects[0].clone(), objects[1].clone()),
            _ => {
                objects.sort_unstable_by(comparator);
                let mid = len / 2;
                let (left_slice, right_slice) = objects.split_at_mut(mid);
                let left_node = Arc::new(BvhNode::new(left_slice)) as Arc<dyn Hittable>;
                let right_node = Arc::new(BvhNode::new(right_slice)) as Arc<dyn Hittable>;
                (left_node, right_node)
            }
        };

        BvhNode { left, right, bbox }
    }

    /// Builds a BVH tree from a HittableList.
    /// It takes ownership of the list's objects (moves them) to avoid copying.
    pub fn from_list(list: HittableList) -> BvhNode {
        let mut objects: Vec<Arc<dyn Hittable>> =
            list.into_objects().into_iter().map(Arc::from).collect();
        Self::new(&mut objects)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, &ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(
            r,
            Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
