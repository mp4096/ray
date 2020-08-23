use crate::ray::Ray;
use crate::util::degrees_to_radians;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn default(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0_f64 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).make_unit_vector();
        let u = vup.cross(&w).make_unit_vector();
        let v = w.cross(&u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = look_from - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        // let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        // let vertical = Vec3::new(0.0, viewport_height, 0.0);

        let lens_radius = aperture / 2.0;

        Camera {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
