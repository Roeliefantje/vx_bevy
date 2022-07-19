//
// This is a rust port of SDF shape functions from Inigo Quilez's web site available at the following URL :
//   https://iquilezles.org/articles/distfunctions/
//

use bevy::math::{vec2, Vec2, Vec3, Vec3Swizzles};

#[allow(dead_code)]
pub fn sdf_sphere(p: Vec3, radius: f32) -> f32 {
    p.length() - radius
}

#[allow(dead_code)]
pub fn sdf_torus(p: Vec3, t: Vec2) -> f32 {
    let q = vec2(p.xz().length() - t.x, p.y);
    q.length() - t.y
}

#[allow(dead_code)]
pub fn sdf_capped_cylinder(p: Vec3, h: f32, r: f32) -> f32 {
    let d = vec2(p.xz().length(), p.y).abs() - vec2(h, r);
    d.x.max(d.y).min(0.) + d.max(Vec2::ZERO).length()
}
