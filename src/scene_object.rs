use crate::color::Color;
use crate::intersection::Intersectable;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector3::Vector3;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub albedo: f32,
}

pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub color: Color,
    pub albedo: f32,
}

pub enum SceneObject {
    Sphere(Sphere),
    Plane(Plane),
}
impl SceneObject {
    pub fn color(&self) -> &Color {
        match *self {
            SceneObject::Sphere(ref s) => &s.color,
            SceneObject::Plane(ref p) => &p.color,
        }
    }
    pub fn albedo(&self) -> f32 {
        match *self {
            SceneObject::Sphere(ref s) => s.albedo,
            SceneObject::Plane(ref p) => p.albedo,
        }
    }
}

impl Intersectable for SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            SceneObject::Sphere(ref s) => s.intersect(ray),
            SceneObject::Plane(ref p) => p.intersect(ray),
        }
    }
    fn normal_at(&self, hit_point: &Point) -> Vector3 {
        match *self {
            SceneObject::Sphere(ref s) => s.normal_at(hit_point),
            SceneObject::Plane(ref p) => p.normal_at(hit_point),
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let ray_origin_to_sphere: Vector3 = self.center - ray.origin;
        let adj = ray_origin_to_sphere.dot(&ray.direction);
        let d2 = ray_origin_to_sphere.dot(&ray_origin_to_sphere) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }
    fn normal_at(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom.abs() > 1e-6 {
            let camera_to_plane = self.origin - ray.origin;
            let distance = camera_to_plane.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
    fn normal_at(&self, _: &Point) -> Vector3 {
        self.normal
    }
}
