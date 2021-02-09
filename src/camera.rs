use crate::*;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

pub struct CameraArgs {
    pub aspect_ratio: float,
    pub viewport_height: float,
    pub focal_length: float,
    pub origin : Vec3
}

impl Camera {
    pub fn new(args: CameraArgs) -> Camera {
        let origin = args.origin;
        let viewport_width = args.viewport_height * args.aspect_ratio;
        let horizontal = vec3(viewport_width, 0.0, 0.0);
        let vertical = vec3(0.0, args.viewport_height, 0.0);
        let lower_left_corner = origin - 0.5*horizontal - 0.5*vertical - vec3(0.0, 0.0, args.focal_length);
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, s: float, t: float) -> Ray {
        Ray::through_points(self.origin, self.lower_left_corner + s*self.horizontal + t*self.vertical)
    }
}