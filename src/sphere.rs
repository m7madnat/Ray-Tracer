use crate::vec::Vec3;
use crate::ray::Ray;
use crate::hit::{ HitRecord, Hittable };

pub struct Sphere {
    pub center: Vec3, 
    pub radius: f32, 
    pub color: Vec3, 
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, color: Vec3) -> Self {
        Self { center, radius, color }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut t = (-b - sqrt_d) / (2.0 * a);
        if t < t_min || t > t_max {
            t = (-b + sqrt_d) / (2.0 * a);
            if t < t_min || t > t_max {
                return None;
            }
        }

        let point = ray.point_at_parameter(t);
        let normal = (point - self.center) / self.radius;

        Some(HitRecord {
            point,
            normal,
            t,
            color: self.color,
        })
    }
}
