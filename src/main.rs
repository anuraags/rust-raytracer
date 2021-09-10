extern crate image;
mod color;
mod intersection;
mod light;
mod point;
mod ray;
mod scene;
mod scene_object;
mod vector3;

use crate::color::Color;
use crate::intersection::Intersectable;
use crate::intersection::Intersection;
use crate::light::Light;
use crate::point::Point;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::scene_object::Plane;
use crate::scene_object::SceneObject;
use crate::scene_object::Sphere;
use crate::vector3::Vector3;

pub use image::{GenericImage, GenericImageView, Pixel, Rgba};

fn to_rgba(color: &Color) -> Rgba<u8> {
    Rgba::from_channels(
        (color.red * 255.0) as u8,
        (color.green * 255.0) as u8,
        (color.blue * 255.0) as u8,
        255,
    )
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let mut color = Color {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
    };
    for light in &scene.lights {
        let hit_point = ray.origin + (ray.direction * intersection.distance);
        let surface_normal = intersection.object.normal_at(&hit_point);
        let direction_to_light = -light.direction.normalize();
        let light_reflected = intersection.object.albedo() / std::f32::consts::PI;

        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * scene.shadow_bias),
            direction: direction_to_light,
        };
        let in_light = scene.trace(&shadow_ray).is_none();

        let light_intensity = if in_light { light.intensity } else { 0.0 };
        let light_power =
            (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;

        let this_light_color = intersection.object.color().clone()
            * light.color.clone()
            * light_power
            * light_reflected;
        color = color + this_light_color;
    }
    color.clamp()
}

pub fn render(scene: &Scene) -> image::DynamicImage {
    let mut image = image::DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime_ray(x, y, scene);
            let nearest_object = scene.trace(&ray);

            match nearest_object {
                None => image.put_pixel(x, y, black),
                Some(intersection) => {
                    let color: Color = get_color(scene, &ray, &intersection);
                    // print!("{:?}\n", color);
                    image.put_pixel(x, y, to_rgba(&color));
                }
            }
        }
    }
    image
}

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 5,
        height: 5,
        fov: 90.0,
        shadow_bias: 1e-6,
        scene_objects: vec![],
        lights: vec![Light {
            direction: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            color: Color {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
            },
            intensity: 1.0,
        }],
    };

    let img: image::DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}

fn main() {
    let scene_objects: Vec<SceneObject> = vec![
        SceneObject::Sphere(Sphere {
            center: Point {
                x: -2.0,
                y: 5.0,
                z: -10.0,
            },
            radius: 5.0,
            color: Color {
                red: 0.4,
                green: 0.4,
                blue: 1.0,
            },
            albedo: std::f32::consts::PI,
        }),
        SceneObject::Sphere(Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.4,
            },
            albedo: std::f32::consts::PI,
        }),
        SceneObject::Sphere(Sphere {
            center: Point {
                x: 3.0,
                y: 2.0,
                z: -3.0,
            },
            radius: 2.5,
            color: Color {
                red: 1.0,
                green: 0.4,
                blue: 0.4,
            },
            albedo: std::f32::consts::PI,
        }),
        SceneObject::Plane(Plane {
            origin: Point {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            normal: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            color: Color {
                red: 0.4,
                green: 0.4,
                blue: 0.4,
            },
            albedo: std::f32::consts::PI,
        }),
    ];
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        scene_objects: scene_objects,
        shadow_bias: 1e-6,
        lights: vec![
            Light {
                direction: Vector3 {
                    x: 1.0,
                    y: -2.0,
                    z: -1.0,
                },
                color: Color {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                },
                intensity: 1.0,
            },
            Light {
                direction: Vector3 {
                    x: -1.0,
                    y: -1.0,
                    z: -1.0,
                },
                color: Color {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                },
                intensity: 2.0,
            },
            Light {
                direction: Vector3 {
                    x: 1.0,
                    y: -1.0,
                    z: -1.0,
                },
                color: Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 1.0,
                },
                intensity: 2.0,
            },
        ],
    };
    // loop {
    //     let rendered = render(&scene);
    //     print!("Finished rendering scene!\n");
    // }
    let rendered = render(&scene);
    rendered.save("fractal.png").unwrap();
}
