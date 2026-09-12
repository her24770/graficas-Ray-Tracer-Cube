use crate::ray_intersect::{Intersect, Material, RayIntersect};
use crate::texture::Texture;
use nalgebra_glm::Vec3;
use std::rc::Rc;

pub struct Cube {
    pub center: Vec3,
    /// Ancho, alto y profundidad totales de la caja (no necesariamente igual
    /// en los tres ejes: permite tanto un cubo como una base/piso delgado).
    pub size: Vec3,
    pub texture: Rc<Texture>,
    pub albedo: f32,
    pub uv_scale: f32,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let half = self.size * 0.5;
        let min = self.center - half;
        let max = self.center + half;

        let axes = [
            (
                ray_origin.x,
                ray_direction.x,
                min.x,
                max.x,
                Vec3::new(1.0, 0.0, 0.0),
            ),
            (
                ray_origin.y,
                ray_direction.y,
                min.y,
                max.y,
                Vec3::new(0.0, 1.0, 0.0),
            ),
            (
                ray_origin.z,
                ray_direction.z,
                min.z,
                max.z,
                Vec3::new(0.0, 0.0, 1.0),
            ),
        ];

        let mut t_near = f32::NEG_INFINITY;
        let mut t_far = f32::INFINITY;
        let mut normal = Vec3::new(0.0, 0.0, 0.0);

        for (origin, direction, min_bound, max_bound, axis_normal) in axes {
            if direction.abs() < 1e-8 {
                if origin < min_bound || origin > max_bound {
                    return None;
                }
                continue;
            }

            let inv_direction = 1.0 / direction;
            let mut t1 = (min_bound - origin) * inv_direction;
            let mut t2 = (max_bound - origin) * inv_direction;
            let mut entering_normal = axis_normal * -1.0;

            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
                entering_normal = axis_normal;
            }

            if t1 > t_near {
                t_near = t1;
                normal = entering_normal;
            }

            if t2 < t_far {
                t_far = t2;
            }

            if t_near > t_far {
                return None;
            }
        }

        if t_far < 0.0 {
            return None;
        }

        let t = if t_near > 0.0 { t_near } else { t_far };

        let point = ray_origin + ray_direction * t;
        let local = point - self.center;

        // Coordenadas UV locales a la cara golpeada: se toman los dos ejes
        // distintos al de la normal y se normalizan de [-half, half] a [0, 1].
        let (u, v) = if normal.x.abs() > 0.5 {
            ((local.z / half.z + 1.0) / 2.0, (local.y / half.y + 1.0) / 2.0)
        } else if normal.y.abs() > 0.5 {
            ((local.x / half.x + 1.0) / 2.0, (local.z / half.z + 1.0) / 2.0)
        } else {
            ((local.x / half.x + 1.0) / 2.0, (local.y / half.y + 1.0) / 2.0)
        };

        let u = (u * self.uv_scale).rem_euclid(1.0);
        let v = (v * self.uv_scale).rem_euclid(1.0);

        let diffuse = self.texture.sample(u, v);

        Some(Intersect {
            point,
            normal,
            distance: t,
            material: Material::new(diffuse, self.albedo),
        })
    }
}
