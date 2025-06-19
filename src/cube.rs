use crate::vec::Vec3;
use crate::ray::Ray;
use crate::hit::{HitRecord, Hittable};

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub color: Vec3,
}

impl Cube{
    pub fn new(min:Vec3,max:Vec3,color:Vec3)->Self{
        Cube{ min, max , color}
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut t_min_m = t_min;
        let mut t_max_m = t_max;

        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min_m = t_min_m.max(t0);
            t_max_m = t_max_m.min(t1);

            if t_max_m <= t_min_m {
                return None;
            }
        }

        let t = t_min_m;
        let point = ray.point_at_parameter(t);

        let mut normal = Vec3::new(0.0, 0.0, 0.0);
        let eps = 0.001;
        for i in 0..3 {
            if (point[i] - self.min[i]).abs() < eps {
                normal[i] = -1.0;
            } else if (point[i] - self.max[i]).abs() < eps {
                normal[i] = 1.0;
            }
        }

        Some(HitRecord {
            point,
            normal,
            t,
            color: self.color, // ✅ don't shade here
        })
    }
}
