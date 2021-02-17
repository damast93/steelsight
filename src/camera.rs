use crate::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    aperture: float,
    u : Vec3,
    v : Vec3
}

pub struct CameraArgs {
    pub lookfrom : Vec3,
    pub lookat : Vec3,
    pub vup : Vec3,
    pub vfow_degrees: float,
    pub aspect_ratio: float,
    pub aperture: float,
    pub focus_dist: float
}

impl Camera {
    pub fn new(args: CameraArgs) -> Camera {
        let theta = deg_to_rad(args.vfow_degrees);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * args.aspect_ratio;

        let w = (args.lookfrom - args.lookat).unit();
        let u = args.vup.cross(w).unit();
        let v = w.cross(u);

        let aperture = args.aperture;
        let origin = args.lookfrom;
        let horizontal = args.focus_dist * viewport_width * u;
        let vertical = args.focus_dist * viewport_height * v;
        let lower_left_corner = origin - 0.5*horizontal - 0.5*vertical - args.focus_dist*w;
        Camera { u, v, origin, lower_left_corner, horizontal, vertical, aperture }
    }

    pub fn get_ray(&self, s: float, t: float, rng: &mut Random) -> Ray {
        let r = 0.5 * self.aperture * random::in_unit_disk(rng);
        let offset = r.x * self.u + r.y * self.v;

        Ray::through_points(
            self.origin + offset,
            self.lower_left_corner + s*self.horizontal + t*self.vertical
        )
    }
}