use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;

use crate::hit::{HitRecord, Hittable};

pub struct Sphere {
    centre: Point3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub const fn new(centre: Point3, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            centre,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.centre;
        let a = ray.direction.length_squared();
        let half_b = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let d = discriminant.sqrt();
        let mut root = (-half_b - d) / a;
        // checking both roots, for closest within bounds
        if root < t_min || root > t_max {
            root = (-half_b + d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let point = ray.at(root);
        let normal = (point - self.centre) / self.radius; // divide by radius same as finding unit vector
        let (normal, hit_from_inside) = if normal.dot(ray.direction) > 0.0 {
            // inside sphere, since pointing in same direction
            (-normal, true)
        } else {
            // outside sphere
            (normal, false)
        };
        Some(HitRecord::new(
            point,
            normal,
            root,
            hit_from_inside,
            self.material.as_ref(),
        ))
    }
}
