use crate::{
    hittable::{HitRecord, Hittable},
    optical::material::Material,
    utilities::{Aabb, Interval, Point3, Ray, Vec3},
};

/// A parallelogram defined by its starting corner and two side vectors.
pub struct Quad {
    q: Point3, // starting point
    u: Vec3,   // side vector
    v: Vec3,   // side vector
    w: Vec3,   // normal oriented with length: 1 / area
    mat: Material,
    bbox: Aabb,

    normal: Vec3,
    d: f32, // D in containing plane Ax+By+Cz = D
}

impl Quad {
    /// Creates a 2D quadrilateral, defined by its starting point and two side vectors
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Material) -> Quad {
        let n = Vec3::cross(&u, &v);
        let normal = Vec3::unit_vector(&n);

        Quad {
            q,
            u,
            v,
            w: n / Vec3::dot(&n, &n),
            mat,
            bbox: Aabb::empty(),
            normal,
            d: Vec3::dot(&normal, &q),
        }
        .set_bounding_box()
    }

    fn set_bounding_box(mut self) -> Self {
        // Compute the bounding box of all four vertices.
        let bbox_diagonal1 = Aabb::extrema(&self.q, &(self.q + self.u + self.v));
        let bbox_diagonal2 = Aabb::extrema(&(self.q + self.u), &(self.q + self.v));

        self.bbox = Aabb::merge(&bbox_diagonal1, &bbox_diagonal2);

        self
    }

    fn is_interior(a: f32, b: f32, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit<'a>(&'a self, r: &Ray, ray_t: Interval, rec: &mut HitRecord<'a>) -> bool {
        let denom = Vec3::dot(&self.normal, &r.direction());

        // No hit if the ray is parallel to the plane.
        if f32::abs(denom) < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - Vec3::dot(&self.normal, &r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = Vec3::dot(&self.w, &Vec3::cross(&planar_hitpt_vector, &self.v));
        let beta = Vec3::dot(&self.w, &Vec3::cross(&self.u, &planar_hitpt_vector));

        if !Self::is_interior(alpha, beta, rec) {
            return false;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.

        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(&self.mat);
        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
