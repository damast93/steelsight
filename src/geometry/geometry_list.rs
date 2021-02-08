use crate::geometry::*;

pub struct GeometryList {
    pub objects : Vec<Box<dyn Geometry>>
}

impl GeometryList {
    pub fn new() -> GeometryList {
        GeometryList { objects: Vec::new() }
    }

    // Move `object` into its own Box and into the vector
    // We need to put a 'static type constraint on `object`, i.e. it should not use any short-lived references ...
    // This is because in the Box, it can live as long as it wants
    pub fn push(&mut self, object: impl Geometry + 'static) {
        self.objects.push(Box::new(object))
    }
}

impl Geometry for GeometryList {
    fn hit(&self, ray: Ray, t_min: float, t_max: float) -> Option<HitRecord> {
        let mut best_hit = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                best_hit = Some(hit);
            }
        }
        
        best_hit
    }
}