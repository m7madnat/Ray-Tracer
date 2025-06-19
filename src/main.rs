mod camera;
mod hit;
mod plane;
mod ray;
mod sphere;
mod vec;
mod cylinder;
mod cube;

use std::env;
use hit::{ HitRecord, Hittable };
use cylinder::Cylinder;
use cube::Cube;
use ray::Ray;
use sphere::Sphere;
use vec::Vec3;
// use plane::Plane;
use camera::Camera;

use std::fs::File;
use std::io::Write;

fn main() {
    let image_width = 800;
    let image_height = 600;
    let samples_per_pixel = 100;
    const MAX_DEPTH: u32 = 8;

    let camera = Camera::new();

    let args: Vec<String> = env::args().collect();
    let mode = if args.len() > 1 { args[1].as_str() } else { "all" };

    let mut world: Vec<Box<dyn Hittable>> = Vec::new();
    let half = 0.5;
    match mode {
        "sphere" => {
            world.push(
                Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Vec3::new(0.8, 0.9, 1.0)))
            );
            world.push(
                Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Vec3::new(0.6, 0.7, 0.8)))
            );
        }
        "cylinder" => {
            world.push(
                Box::new(
                    Cylinder::new(Vec3::new(0.0, -0.5, 0.0), 0.3, 1.0, Vec3::new(0.8, 0.9, 1.0))
                )
            );
            world.push(
                Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Vec3::new(0.6, 0.7, 0.8)))
            );
        }
        "cube" => {
            world.push(
                Box::new(
                    Cube::new(
                        Vec3::new(-half, -half, -1.0 - half),
                        Vec3::new(half, half, -1.0 + half),
                        Vec3::new(0.8, 0.9, 1.0)
                    )
                )
            );
            world.push(
                Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Vec3::new(0.6, 0.7, 0.8)))
            );
        }
        _ => {
            world.push(
                Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Vec3::new(0.6, 0.7, 0.8)))
            );
            world.push(
                Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Vec3::new(0.6, 0.7, 0.8)))
            );
            world.push(
                Box::new(
                    Cube::new(
                        Vec3::new(-half, -half, -1.0 - half),
                        Vec3::new(half, half, -1.0 + half),
                        Vec3::new(0.8, 0.9, 1.0)
                    )
                )
            );
            world.push(
                Box::new(
                    Cylinder::new(Vec3::new(-1.0, -0.5, -2.0), 0.3, 1.0, Vec3::new(0.8, 0.9, 1.0))
                )
            );
        }
    }
    let mut file = File::create("output.ppm").unwrap();
    writeln!(file, "P3\n{} {}\n255", image_width, image_height).unwrap();

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = ((i as f32) + rand::random::<f32>()) / ((image_width - 1) as f32);
                let v = ((j as f32) + rand::random::<f32>()) / ((image_height - 1) as f32);
                let r = camera.get_ray(u, v);
                color = color + ray_color(&r, &world, MAX_DEPTH);
            }
            color = (color / (samples_per_pixel as f32)) * 1.5;

            let ir = (255.99 * color.x.clamp(0.0, 1.0)) as i32;
            let ig = (255.99 * color.y.clamp(0.0, 1.0)) as i32;
            let ib = (255.99 * color.z.clamp(0.0, 1.0)) as i32;

            writeln!(file, "{} {} {}", ir, ig, ib).unwrap();
        }
    }

    println!("Done! Output written to output.ppm");
}

fn ray_color(ray: &Ray, world: &Vec<Box<dyn Hittable>>, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let mut closest_t = f32::INFINITY;
    let mut hit_rec = None;
    for obj in world.iter() {
        if let Some(rec) = obj.hit(ray, 0.001, closest_t) {
            closest_t = rec.t;
            hit_rec = Some(rec);
        }
    }

    if let Some(rec) = hit_rec {
        let target_dir = rec.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(rec.point + rec.normal * 0.001, target_dir);
        return rec.color * ray_color(&scattered, world, depth - 1) * 0.5;
    }

    background(ray)
}
fn background(ray: &Ray) -> Vec3 {
    let t = 0.5 * (ray.direction.normalize().y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.8, 0.9, 1.0) * t
}
