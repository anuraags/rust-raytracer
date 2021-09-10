use crate::point::Point;
use crate::ray::Ray;
use crate::scene_object::SceneObject;
use crate::vector3::Vector3;

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a SceneObject,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, object: &'b SceneObject) -> Intersection<'b> {
        // Elided
        Intersection {
            distance: distance,
            object: object,
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn normal_at(&self, hit_point: &Point) -> Vector3;
}
