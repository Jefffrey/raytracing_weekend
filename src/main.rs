mod camera;
mod geometry;
mod hit;
mod material;
mod ray;
mod vec3;

use std::{
    fs::File,
    io::{BufWriter, Write},
    time::Instant,
};

use geometry::Sphere;
use hit::Hittable;
use material::{Dielectric, Lambertian, Material, Metal};
use rand::{thread_rng, Rng};
use ray::Ray;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use vec3::{Point3, Rgb, Vec3};

use crate::camera::Camera;

const WHITE: Rgb = Rgb::from_scalar(1.0);
const LIGHT_BLUE: Rgb = Rgb::new(0.5, 0.7, 1.0);

fn ray_colour(ray: &Ray, depth: u32, world: &[Box<dyn Hittable + Sync>]) -> Rgb {
    match depth {
        0 => Rgb::default(), // max bounces, no colour
        _ => world
            .iter()
            // min 0.001 to account for shadow acne (due to floating point inaccuracy, ignore self hits (since t might not be at exact hit spot))
            .filter_map(|h| h.hit(ray, 0.001, f64::INFINITY))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
            .map(|hr| {
                hr.material
                    .scatter(ray, &hr)
                    .map(|(scattered, attenuation)| {
                        attenuation * ray_colour(&scattered, depth - 1, world)
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_else(|| {
                let t = 0.5 * (ray.direction.unit().y + 1.0); // from [-1.0, 1.0] to [0.0, 1.0]
                WHITE * (1.0 - t) + LIGHT_BLUE * t // linearly interpolate
            }),
    }
}

fn random_scene() -> Vec<Box<dyn Hittable + Sync>> {
    let mut rng = thread_rng();

    let mut world: Vec<Box<dyn Hittable + Sync>> = vec![];
    let ground = Box::new(Lambertian::new(Rgb::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let material_choice = rng.gen::<f64>();
            let centre = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (centre - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Box<dyn Material + Sync> = if material_choice < 0.8 {
                    // diffuse
                    let albedo = Rgb::random() * Rgb::random();
                    Box::new(Lambertian::new(albedo))
                } else if material_choice < 0.95 {
                    // metal
                    let albedo = Rgb::random_in_range(0.5, 1.0);
                    let fuzziness = rng.gen_range(0.0..0.5);
                    Box::new(Metal::new(albedo, fuzziness))
                } else {
                    // glass
                    Box::new(Dielectric::new(1.5))
                };
                world.push(Box::new(Sphere::new(centre, 0.2, material)));
            }
        }
    }

    let material_1 = Box::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Box::new(Lambertian::new(Rgb::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Box::new(Metal::new(Rgb::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    world
}

fn main() {
    let now = Instant::now();

    let max_depth = 50;
    let aspect_ratio = 3.0 / 2.0;
    let samples_per_pixel = 500;

    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let file = File::create("out.png").unwrap();
    let mut encoder = png::Encoder::new(BufWriter::new(file), image_width, image_height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .unwrap()
        .into_stream_writer()
        .unwrap();

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let focus_distance = 10.0;
    let camera = Camera::new(
        look_from,
        look_at,
        up,
        20.0,
        aspect_ratio,
        aperture,
        focus_distance,
    );

    let world = random_scene();

    for j in (0..image_height).rev() {
        let row_bytes = (0..image_width)
            .into_par_iter()
            .flat_map(|i| {
                let mut rng = thread_rng();
                let mut colour = Vec3::default();
                // multiple rays per pixel for AA
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / ((image_width - 1) as f64);
                    let v = (j as f64 + rng.gen::<f64>()) / ((image_height - 1) as f64);
                    let ray = camera.get_ray(u, v);
                    colour += ray_colour(&ray, max_depth, &world);
                }
                colour /= samples_per_pixel as f64;
                // sqrt for gamma correction, gamma = 2.0 (raise colour to 1/gamma)
                colour.sqrt().as_bytes()
            })
            .collect::<Vec<u8>>();
        writer.write_all(&row_bytes).unwrap();
        println!("[{}/{}] rows", image_height - j, image_height);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
