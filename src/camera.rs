use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {

        // ////////////////////////////////////////////////////
        // let image_width = 800.0;
        // let image_height = 600.0;
        // let aspect_ratio = image_width / image_height;
        // You can customize this
        // let look_from = Vec3::new(0.0, 6.0, -1.0);
        // let look_at = Vec3::new(0.0, 0.0, -1.0);

        // look_at_camera(look_from, look_at, aspect_ratio)
        ////////////////////////////////////////////////////

        //top view
        Camera {
            origin: Vec3::new(0.0, 100.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, 20.0, -1.5),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 0.0, 3.0),
        }

        //front view
        // Camera {
        //     origin: Vec3::new(0.0, 0.0, 4.0),
        //     lower_left_corner: Vec3::new(-2.0, -1.5, -1.0),
        //     horizontal: Vec3::new(4.0, 0.0, 0.0),
        //     vertical: Vec3::new(0.0, 3.0, 0.0),
        // }

        //right view
        // Camera {
        //     origin: Vec3::new(5.0, 0.0, 0.0),
        //     lower_left_corner: Vec3::new(2.0, -1.0, -2.0),
        //     horizontal: Vec3::new(0.0, 0.0, 4.0),
        //     vertical: Vec3::new(0.0, 3.0, 0.0),
        // }
}

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

// pub fn look_at_camera(look_from: Vec3, look_at: Vec3, aspect_ratio: f32) -> Camera {
    
//     let vfov: f32 = 40.0;
//     let vup = Vec3::new(0.0, 0.0, -1.0);
//     let theta = vfov.to_radians();
//     let h = (theta / 2.0).tan();
//     let viewport_height = 2.0 * h;
//     let viewport_width = aspect_ratio * viewport_height;

//     let w = (look_from - look_at).normalize(); // camera backward
//     let u = vup.cross(w).normalize(); // camera right
//     let v = w.cross(u); // camera up

//     let origin = look_from;
//     let horizontal = u * viewport_width;
//     let vertical = v * viewport_height;
//     let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

//     Camera {
//         origin,
//         lower_left_corner,
//         horizontal,
//         vertical,
//     }
// }
