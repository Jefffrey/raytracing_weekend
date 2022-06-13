use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3, // must be against hit direction, i.e. dot product with dir is negative, also must be normalized
    pub t: f64,       // of the ray equation
    pub hit_from_inside: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Point3,
        normal: Vec3,
        t: f64,
        hit_from_inside: bool,
        material: &'a dyn Material,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            hit_from_inside,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
