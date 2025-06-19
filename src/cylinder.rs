use crate::{ vec::Vec3, ray::Ray, hit::{ HitRecord, Hittable } };

pub struct Cylinder {
    pub center: Vec3,
    pub radius: f32,
    pub height: f32,
    pub color: Vec3,
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f32, height: f32, color: Vec3) -> Self {
        Self { center, radius, height, color }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = t_max;

        let oc = ray.origin - self.center;
        let dir = ray.direction;

        // الجدار الجانبي
        let a = dir.x * dir.x + dir.z * dir.z;
        let b = 2.0 * (oc.x * dir.x + oc.z * dir.z);
        let c = oc.x * oc.x + oc.z * oc.z - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let sqrt_d = discriminant.sqrt();
            for &sign in &[-1.0, 1.0] {
                let t = (-b + sign * sqrt_d) / (2.0 * a);
                if t < t_min || t > closest_t {
                    continue;
                }

                let point = ray.point_at_parameter(t);
                if point.y >= self.center.y && point.y <= self.center.y + self.height {
                    let normal = Vec3 {
                        x: (point.x - self.center.x) / self.radius,
                        y: 0.0,
                        z: (point.z - self.center.z) / self.radius,
                    };
                    closest_t = t;
                    closest_hit = Some(HitRecord {
                        t,
                        point,
                        normal,
                        color: self.color,
                    });
                }
            }
        }

        // القاعدة السفلية (y = center.y)
        let t_bottom = (self.center.y - ray.origin.y) / ray.direction.y;
        if t_bottom >= t_min && t_bottom <= closest_t {
            let p = ray.point_at_parameter(t_bottom);
            let dx = p.x - self.center.x;
            let dz = p.z - self.center.z;
            if dx * dx + dz * dz <= self.radius * self.radius {
                closest_t = t_bottom;
                closest_hit = Some(HitRecord {
                    t: t_bottom,
                    point: p,
                    normal: Vec3::new(0.0, -1.0, 0.0),
                    color: self.color,
                });
            }
        }

        // القاعدة العلوية (y = center.y + height)
        let top_y = self.center.y + self.height;
        let t_top = (top_y - ray.origin.y) / ray.direction.y;
        if t_top >= t_min && t_top <= closest_t {
            let p = ray.point_at_parameter(t_top);
            let dx = p.x - self.center.x;
            let dz = p.z - self.center.z;
            if dx * dx + dz * dz <= self.radius * self.radius {
                closest_t = t_top;
                closest_hit = Some(HitRecord {
                    t: t_top,
                    point: p,
                    normal: Vec3::new(0.0, 1.0, 0.0),
                    color: self.color,
                });
            }
        }

        closest_hit
    }
}
