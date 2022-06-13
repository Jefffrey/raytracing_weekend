use rand::{thread_rng, Rng};

use crate::{
    hit::HitRecord,
    ray::Ray,
    vec3::{Rgb, Vec3},
};

pub trait Material: Sync {
    // returns scattered ray (and colour attenuation), if any
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<(Ray, Rgb)>;
}

pub struct Lambertian {
    albedo: Rgb,
}

impl Lambertian {
    pub const fn new(albedo: Rgb) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hr: &HitRecord) -> Option<(Ray, Rgb)> {
        // unit circle tangent to hit point, in direction of normal (which is against ray)
        // send random ray to somewhere in this unit circle for diffuse ray collection
        // convert random vec to unit for Lambertian distribution (pick along surface of unit sphere)
        let scatter_dir = hr.normal + Vec3::random_in_unit_sphere().unit();
        // in case random vec is opposite to normal = 0 vec
        let scatter_dir = if scatter_dir.near_zero() {
            hr.normal
        } else {
            scatter_dir
        };
        Some((Ray::new(hr.point, scatter_dir), self.albedo))
    }
}

pub struct Metal {
    albedo: Rgb,
    fuzziness: f64, // in range [0.0, 1.0], but can't enforce at compile time, so please play nice
}

impl Metal {
    pub const fn new(albedo: Rgb, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<(Ray, Rgb)> {
        let reflected_dir = ray.direction.unit().reflect_across(hr.normal);
        let scatter_r = Ray::new(
            hr.point,
            // randomise endpoint of reflect a bit depending on how fuzzy material is (high fuzz = high variance/blur)
            reflected_dir + Vec3::random_in_unit_sphere() * self.fuzziness,
        );
        if scatter_r.direction.dot(hr.normal) > 0.0 {
            // proper reflection
            Some((scatter_r, self.albedo))
        } else {
            // hitting from under normal
            None
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub const fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_index: f64) -> f64 {
        // Schlick's approximation
        let r0 = (1.0 - ref_index) / (1.0 + ref_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<(Ray, Rgb)> {
        let refraction_ratio = if hr.hit_from_inside {
            self.index_of_refraction
        } else {
            // front face
            1.0 / self.index_of_refraction
        };

        let unit_dir = ray.direction.unit();

        let cos_theta = hr.normal.dot(-unit_dir).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let refracted_dir = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > thread_rng().gen()
        {
            unit_dir.reflect_across(hr.normal)
        } else {
            unit_dir.refract_across(hr.normal, refraction_ratio)
        };

        Some((Ray::new(hr.point, refracted_dir), Vec3::from_scalar(1.0)))
    }
}
