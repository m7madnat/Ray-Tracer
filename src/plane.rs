use crate::vec::Vec3;
use crate::ray::Ray;
use crate::hit::{ HitRecord, Hittable };

pub struct Plane {
    pub point: Vec3, 
    pub normal: Vec3, 
    pub color: Vec3, 
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, color: Vec3) -> Self {
        Plane {
            point,
            normal: normal.normalize(),
            color,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() > 1e-6 {
            let t = (self.point - ray.origin).dot(self.normal) / denom;
            if t >= t_min && t <= t_max {
                let mut point = ray.point_at_parameter(t);

                let bump = (point.x * 10.0).sin() * 0.02 + (point.z * 10.0).cos() * 0.02;
                point.y += bump;

                return Some(HitRecord {
                    point,
                    normal: self.normal,
                    t,
                    color: self.color,
                });
            }
        }
        None
    }
}
