use crate::util::{
    Arc, HitRecord, Hittable, HittableList, Interval, Material, Point3, Ray, Vec3, AABB,
};

trait plane: Send + Sync {
    fn set_bounding_box(&mut self);
    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool;
}

pub struct Quad {
    bbox: AABB,
    Q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Option<Arc<dyn Material>>,
    normal: Vec3,
    D: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(Q: Point3, u: Vec3, v: Vec3, mat: Option<Arc<dyn Material>>) -> Self {
        let n = u.cross(&v);
        let mut ret = Self {
            bbox: AABB::default(),
            Q,
            u,
            v,
            mat,
            normal: n.normalize(),
            D: n.normalize().dot(&(Q.to_vec3())),
            w: n / n.dot(&n),
        };
        ret.set_bounding_box();
        ret
    }
}

impl plane for Quad {
    fn set_bounding_box(&mut self) {
        let bbox1 = AABB::new_by_point(self.Q, self.Q + self.u + self.v);
        let bbox2 = AABB::new_by_point(self.Q + self.u, self.Q + self.v);
        self.bbox = AABB::new_by_aabb(bbox1, bbox2)
    }
    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if (!unit_interval.contain(a) || !unit_interval.contain(b)) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&ray.direction());
        if denom.abs() < 1e-8 {
            return false;
        }
        let t = (self.D - self.normal.dot(&(ray.origin().to_vec3()))) / denom;
        if (!ray_t.contain(t)) {
            return false;
        }
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.Q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));
        if !Quad::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.as_ref().map(Arc::clone);
        rec.set_face_normal(ray, self.normal);
        true
    }

    fn display(&self) {
        println!("Quadrilateral!");
    }

    fn get_material(&self) -> Option<Arc<dyn Material>> {
        self.mat.as_ref().map(Arc::clone)
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}

pub struct Triangle {
    bbox: AABB,
    Q: Point3,
    u: Vec3,
    v: Vec3,
    tex_ordinates: [[f64; 2]; 3],
    mat: Option<Arc<dyn Material>>,
    normal: Vec3,
    D: f64,
    w: Vec3,
}

impl Triangle {
    pub fn new(
        Q: Point3,
        u: Vec3,
        v: Vec3,
        tex_ordinates: [[f64; 2]; 3],
        mat: Option<Arc<dyn Material>>,
    ) -> Self {
        let n = u.cross(&v);
        let mut ret = Self {
            bbox: AABB::default(),
            Q,
            u,
            v,
            tex_ordinates,
            mat,
            normal: n.normalize(),
            D: n.normalize().dot(&(Q.to_vec3())),
            w: n / n.dot(&n),
        };
        ret.set_bounding_box();
        ret
    }
    fn vector_sub(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
        [a[0] - b[0], a[1] - b[1]]
    }
}

impl plane for Triangle {
    fn set_bounding_box(&mut self) {
        let bbox1 = AABB::new_by_point(self.Q, self.Q + self.u);
        let bbox2 = AABB::new_by_point(self.Q, self.Q + self.v);
        self.bbox = AABB::new_by_aabb(bbox1, bbox2)
    }
    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if (a < 0.0 || b < 0.0 || !unit_interval.contain(a + b)) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&ray.direction());
        if denom.abs() < 1e-8 {
            return false;
        }
        let t = (self.D - self.normal.dot(&(ray.origin().to_vec3()))) / denom;
        if (!ray_t.contain(t)) {
            return false;
        }
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.Q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));
        if !Triangle::is_interior(alpha, beta, rec) {
            return false;
        }

        // compute the texture coordinates
        let tex_a_vec = Triangle::vector_sub(self.tex_ordinates[1], self.tex_ordinates[0]);
        let tex_b_vec = Triangle::vector_sub(self.tex_ordinates[2], self.tex_ordinates[0]);
        rec.u = self.tex_ordinates[0][0] + rec.u * tex_a_vec[0] + rec.v * tex_b_vec[0];
        rec.v = self.tex_ordinates[0][1] + rec.u * tex_a_vec[1] + rec.v * tex_b_vec[1];

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.as_ref().map(Arc::clone);
        rec.set_face_normal(ray, self.normal);
        true
    }

    fn display(&self) {
        println!("Triangle!");
    }

    fn get_material(&self) -> Option<Arc<dyn Material>> {
        self.mat.as_ref().map(Arc::clone)
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}

// u and v should be orthogonal
// Q stand for the center
pub struct Disk {
    bbox: AABB,
    Q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Option<Arc<dyn Material>>,
    normal: Vec3,
    D: f64,
    w: Vec3,
}

impl Disk {
    pub fn new(Q: Point3, u: Vec3, v: Vec3, mat: Option<Arc<dyn Material>>) -> Self {
        let n = u.cross(&v);
        let mut ret = Self {
            bbox: AABB::default(),
            Q,
            u,
            v,
            mat,
            normal: n.normalize(),
            D: n.normalize().dot(&(Q.to_vec3())),
            w: n / n.dot(&n),
        };
        ret.set_bounding_box();
        ret
    }
}

impl plane for Disk {
    fn set_bounding_box(&mut self) {
        let radius = self.u.length();
        self.bbox = AABB::new_by_point(
            self.Q - Vec3::new(radius, radius, radius),
            self.Q + Vec3::new(radius, radius, radius),
        );
    }
    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        if (a * a + b * b > 1.0) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Disk {
    fn hit(&self, ray: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&ray.direction());
        if denom.abs() < 1e-8 {
            return false;
        }
        let t = (self.D - self.normal.dot(&(ray.origin().to_vec3()))) / denom;
        if (!ray_t.contain(t)) {
            return false;
        }
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.Q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));
        if !Disk::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.as_ref().map(Arc::clone);
        rec.set_face_normal(ray, self.normal);
        true
    }

    fn display(&self) {
        println!("Disk!");
    }

    fn get_material(&self) -> Option<Arc<dyn Material>> {
        self.mat.as_ref().map(Arc::clone)
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}

pub fn get_box(a: Point3, b: Point3, mat: Option<Arc<dyn Material>>) -> Arc<HittableList> {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        mat.as_ref().map(Arc::clone),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, max.z),
        dz * (-1.0),
        dy,
        mat.as_ref().map(Arc::clone),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, min.z),
        dx * (-1.0),
        dy,
        mat.as_ref().map(Arc::clone),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dz,
        dy,
        mat.as_ref().map(Arc::clone),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, max.y, max.z),
        dx,
        dz * (-1.0),
        mat.as_ref().map(Arc::clone),
    )));
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dz,
        mat.as_ref().map(Arc::clone),
    )));

    Arc::new(sides)
}
