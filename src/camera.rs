use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    // orthonormal base
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        up: Vec3,
        vertical_fov_degrees: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let h = (vertical_fov_degrees.to_radians() / 2.0).tan();

        // aka the screen
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // orthonormal base
        let w = (look_from - look_at).unit(); // camera looks down -w
        let u = up.cross(w).unit(); // side
        let v = w.cross(u); // up

        let origin = look_from;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v + rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }
}
