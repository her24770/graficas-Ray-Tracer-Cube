use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::Vec3;

pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let half = self.size / 2.0;
        let min = self.center - Vec3::new(half, half, half);
        let max = self.center + Vec3::new(half, half, half);

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

        Some(Intersect {
            point,
            normal,
            distance: t,
            material: self.material,
        })
    }
}
