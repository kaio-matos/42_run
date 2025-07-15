use crate::math::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn from(vertices: &Vec<Vec4>) -> AABB {
        let mut aabb = AABB {
            min: Vec3::new(vertices[0].x, vertices[0].y, vertices[0].z),
            max: Vec3::new(vertices[0].x, vertices[0].y, vertices[0].z),
        };

        for vec in vertices {
            aabb.min.x = f32::min(aabb.min.x, vec.x);
            aabb.min.y = f32::min(aabb.min.y, vec.y);
            aabb.min.z = f32::min(aabb.min.z, vec.z);

            aabb.max.x = f32::max(aabb.max.x, vec.x);
            aabb.max.y = f32::max(aabb.max.y, vec.y);
            aabb.max.z = f32::max(aabb.max.z, vec.z);
        }

        aabb
    }
}
