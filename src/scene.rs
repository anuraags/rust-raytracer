use crate::intersection::Intersectable;
use crate::intersection::Intersection;
use crate::light::Light;
use crate::ray::Ray;
use crate::scene_object::SceneObject;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub scene_objects: Vec<SceneObject>,
    pub lights: Vec<Light>,
    pub shadow_bias: f64,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        let intersections = self
            .scene_objects
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)));
        intersections.min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
