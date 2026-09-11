mod camera;
mod color;
mod cube;
mod framebuffer;
mod light;
mod ray_intersect;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{dot, normalize, Vec3};
use std::f32::consts::PI;
use std::time::Duration;

use crate::camera::Camera;
use crate::color::Color;
use crate::cube::Cube;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::ray_intersect::{Intersect, Material, RayIntersect};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const FOV: f32 = PI / 3.0;

const ROTATION_SPEED: f32 = PI / 60.0;

const AMBIENT_INTENSITY: f32 = 0.08;

fn sky_color(ray_direction: &Vec3) -> Color {
    let top = Color::new(120, 170, 230);
    let bottom = Color::new(15, 25, 70);

    let t = (ray_direction.y * 0.5 + 0.5).clamp(0.0, 1.0);

    top * t + bottom * (1.0 - t)
}

pub fn shade(intersect: &Intersect, light: &Light) -> Color {
    let light_direction = (light.position - intersect.point).normalize();

    let diffuse_intensity = dot(&intersect.normal, &light_direction).max(0.0);

    intersect.material.diffuse
        * ((AMBIENT_INTENSITY + diffuse_intensity * light.intensity) * intersect.material.albedo)
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Box<dyn RayIntersect>],
    light: &Light,
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
        Some(intersect) => shade(&intersect, light),
        None => sky_color(ray_direction),
    }
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Box<dyn RayIntersect>],
    camera: &Camera,
    light: &Light,
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

            framebuffer
                .set_current_color(cast_ray(&camera.eye, &ray_direction, objects, light).to_hex());
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Raytracer Cube", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let purple_rubber = Material::new(Color::new(120, 30, 180), 0.9);

    let objects: Vec<Box<dyn RayIntersect>> = vec![Box::new(Cube {
        center: Vec3::new(0.0, 0.0, 0.0),
        size: 1.6,
        material: purple_rubber,
    })];

    let light = Light::new(Vec3::new(4.0, 6.0, 5.0), Color::new(255, 255, 255), 1.2);

    let mut camera = Camera::new(
        Vec3::new(0.0, 1.2, 4.5),
        Vec3::new(0.0, 0.0, 0.0),
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
            render(&mut framebuffer, &objects, &camera, &light);
            camera_moved = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
