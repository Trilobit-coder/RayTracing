use crate::{
    hittable::{HitRecord, Hittable, HittableList, Quad},
    optical::material::Material,
    utilities::{Aabb, Interval, Point3, Ray, Vec3},
};

/// A 3D block (six sides) primitive.
pub struct Block {
    sides: HittableList,
}

impl Block {
    /// Returns the 3D block (six sides) that contains the two opposite vertices a & b.
    pub fn new(a: Point3, b: Point3, mat: Material) -> Block {
        let mut sides = HittableList::new();

        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min = Point3::new(
            f32::min(a.x(), b.x()),
            f32::min(a.y(), b.y()),
            f32::min(a.z(), b.z()),
        );
        let max = Point3::new(
            f32::max(a.x(), b.x()),
            f32::max(a.y(), b.y()),
            f32::max(a.z(), b.z()),
        );

        let dx = Vec3::new(max.x() - min.x(), 0., 0.);
        let dy = Vec3::new(0., max.y() - min.y(), 0.);
        let dz = Vec3::new(0., 0., max.z() - min.z());

        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            mat.clone(),
        ))); // front
        sides.add(Box::new(Quad::new(
            Point3::new(max.x(), min.y(), max.z()),
            -dz,
            dy,
            mat.clone(),
        ))); // right
        sides.add(Box::new(Quad::new(
            Point3::new(max.x(), min.y(), min.z()),
            -dx,
            dy,
            mat.clone(),
        ))); // back
        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dz,
            dy,
            mat.clone(),
        ))); // left
        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), max.y(), max.z()),
            dx,
            -dz,
            mat.clone(),
        ))); // top
        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            mat,
        ))); // bottom

        Block { sides }
    }
}

impl Hittable for Block {
    fn hit<'a>(&'a self, r: &Ray, ray_t: Interval, rec: &mut HitRecord<'a>) -> bool {
        self.sides.hit(r, ray_t, rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.sides.bounding_box()
    }

    fn has_motion(&self) -> bool {
        self.sides.has_motion()
    }
}
