use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable, HittableList},
    utilities::{Aabb, Interval, Ray},
};

/// A node in a BVH tree, holding left/right subtrees and their bounding boxes.
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    left_bbox: Aabb,
    right_bbox: Aabb,
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
        let (left, right, left_bbox, right_bbox) = match len {
            1 => (objects[0].clone(), objects[0].clone(), bbox, bbox),
            2 => (
                objects[0].clone(),
                objects[1].clone(),
                objects[0].bounding_box(),
                objects[1].bounding_box(),
            ),
            _ => {
                objects.sort_unstable_by(comparator);
                let mid = len / 2;
                let (left_slice, right_slice) = objects.split_at_mut(mid);
                let left_node = BvhNode::new(left_slice);
                let right_node = BvhNode::new(right_slice);
                let left_bbox = left_node.bounding_box();
                let right_bbox = right_node.bounding_box();
                (
                    Arc::new(left_node) as Arc<dyn Hittable>,
                    Arc::new(right_node) as Arc<dyn Hittable>,
                    left_bbox,
                    right_bbox,
                )
            }
        };

        BvhNode {
            left,
            right,
            left_bbox,
            right_bbox,
            bbox,
        }
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
    fn hit<'a>(&'a self, r: &Ray, ray_t: Interval, rec: &mut HitRecord<'a>) -> bool {
        if !self.bbox.hit(r, &ray_t) {
            return false;
        }

        // Visit the child whose bounding box center lies nearer the ray origin first, so that
        // `rec.t` tightens early and the farther child is rejected more often.
        let left_first = (self.left_bbox.centroid() - r.origin()).length_squared()
            <= (self.right_bbox.centroid() - r.origin()).length_squared();
        let (near, far) = if left_first {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        };

        let hit_near = near.hit(r, ray_t, rec);

        // A single-object node stores the same object in both children; skip the duplicate test.
        if Arc::ptr_eq(near, far) {
            return hit_near;
        }

        let hit_far = far.hit(
            r,
            Interval::new(ray_t.min, if hit_near { rec.t } else { ray_t.max }),
            rec,
        );

        hit_near || hit_far
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn has_motion(&self) -> bool {
        self.left.has_motion() || self.right.has_motion()
    }
}
