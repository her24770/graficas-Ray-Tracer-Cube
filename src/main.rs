mod camera;
mod color;
mod cube;
mod framebuffer;
mod light;
mod ray_intersect;
mod texture;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{dot, normalize, Vec3};
use std::f32::consts::PI;
use std::rc::Rc;
use std::time::Duration;

use crate::camera::Camera;
use crate::color::Color;
use crate::cube::Cube;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::texture::Texture;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const FOV: f32 = PI / 3.0;

const ROTATION_SPEED: f32 = PI / 60.0;

const AMBIENT_INTENSITY: f32 = 0.05;

const SHADOW_BIAS: f32 = 1e-3;

fn sky_color(ray_direction: &Vec3) -> Color {
    let top = Color::new(6, 4, 16);
    let horizon = Color::new(55, 12, 55);

    let t = (ray_direction.y * 0.5 + 0.5).clamp(0.0, 1.0);

    top * t + horizon * (1.0 - t)
}

pub fn cast_shadow(
    intersect: &Intersect,
    light_direction: &Vec3,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> bool {
    let shadow_ray_origin = intersect.point + intersect.normal * SHADOW_BIAS;
    let light_distance = (light.position - intersect.point).magnitude();

    objects.iter().any(|object| {
        object
            .ray_intersect(&shadow_ray_origin, light_direction)
            .is_some_and(|blocker| blocker.distance < light_distance)
    })
}

pub fn shade(intersect: &Intersect, lights: &[Light], objects: &[Box<dyn RayIntersect>]) -> Color {
    let mut total = intersect.material.diffuse * AMBIENT_INTENSITY;

    for light in lights {
        let light_direction = (light.position - intersect.point).normalize();

        if cast_shadow(intersect, &light_direction, light, objects) {
            continue;
        }

        let diffuse_intensity = dot(&intersect.normal, &light_direction).max(0.0);

        total = total
            + intersect.material.diffuse * light.color * (diffuse_intensity * light.intensity);
    }

    total * intersect.material.albedo
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Box<dyn RayIntersect>],
    lights: &[Light],
) -> Color {
    let mut closest: Option<Intersect> = None;

    for object in objects {
        if let Some(intersect) = object.ray_intersect(ray_origin, ray_direction) {
            if closest.is_none_or(|current| intersect.distance < current.distance) {
                closest = Some(intersect);
            }
        }
    }

    match closest {
        Some(intersect) => shade(&intersect, lights, objects),
        None => sky_color(ray_direction),
    }
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Box<dyn RayIntersect>],
    camera: &Camera,
    lights: &[Light],
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    let perspective_scale = (FOV / 2.0).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let ray_direction = camera.basis_change(&ray_direction);

            framebuffer.set_current_color(
                cast_ray(&camera.eye, &ray_direction, objects, lights).to_hex(),
            );
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Raytracer Cube", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let stone_texture = Rc::new(Texture::load("assets/stone.png"));
    let floor_texture = Rc::new(Texture::load("assets/floor.png"));

    let cube_size = 1.6;
    let cube_half = cube_size / 2.0;
    let floor_thickness = 0.4;

    let objects: Vec<Box<dyn RayIntersect>> = vec![
        Box::new(Cube {
            center: Vec3::new(0.0, 0.0, 0.0),
            size: Vec3::new(cube_size, cube_size, cube_size),
            texture: stone_texture,
            albedo: 0.9,
            uv_scale: 1.0,
        }),
        Box::new(Cube {
            center: Vec3::new(0.0, -cube_half - floor_thickness / 2.0, 0.0),
            size: Vec3::new(12.0, floor_thickness, 12.0),
            texture: floor_texture,
            albedo: 0.9,
            uv_scale: 6.0,
        }),
    ];

    let lights = vec![
        Light::new(Vec3::new(3.0, 5.0, 4.0), Color::new(210, 220, 255), 1.0),
        Light::new(Vec3::new(-4.0, 1.2, -3.0), Color::new(255, 0, 180), 1.4),
        Light::new(Vec3::new(4.0, 1.2, -3.0), Color::new(0, 220, 255), 1.4),
    ];

    let mut camera = Camera::new(
        Vec3::new(0.0, 1.6, 5.0),
        Vec3::new(0.0, -0.2, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut camera_moved = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let orbit = [
            (Key::Left, ROTATION_SPEED, 0.0),
            (Key::Right, -ROTATION_SPEED, 0.0),
            (Key::Up, 0.0, -ROTATION_SPEED),
            (Key::Down, 0.0, ROTATION_SPEED),
        ];

        for (key, delta_yaw, delta_pitch) in orbit {
            if window.is_key_down(key) {
                camera.orbit(delta_yaw, delta_pitch);
                camera_moved = true;
            }
        }

        if camera_moved {
            render(&mut framebuffer, &objects, &camera, &lights);
            camera_moved = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
