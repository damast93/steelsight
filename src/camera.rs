use crate::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

pub struct CameraArgs {
    pub aspect_ratio: float,
    pub vfow_degrees: float,
    pub focal_length: float,
    pub lookfrom : Vec3,
    pub lookat : Vec3,
    pub vup : Vec3
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

        let origin = args.lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - 0.5*horizontal - 0.5*vertical - w;
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, s: float, t: float) -> Ray {
        Ray::through_points(self.origin, self.lower_left_corner + s*self.horizontal + t*self.vertical)
    }
}