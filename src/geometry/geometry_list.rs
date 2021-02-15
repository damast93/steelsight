use crate::geometry::*;

// Design decision: The geometry list owns its child geometries
// We need a 'scene lifetime constraint, because those might still borrow materials etc.
pub struct GeometryList<'scene> {
    pub objects : Vec<Box<dyn Geometry + 'scene>>
}

impl<'scene> GeometryList<'scene> {
    pub fn new() -> GeometryList<'scene> {
        GeometryList { objects: Vec::new() }
    }

    // Move `object` into its own Box and into the vector
    // We need the 'scene lifetime constraint, because Geometries might borrow
    // references to e.g. materials which only live as long as the scene  
    pub fn push(&mut self, object: impl Geometry + 'scene) {
        self.objects.push(Box::new(object))
    }
}

impl<'scene> Geometry for GeometryList<'scene> {
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