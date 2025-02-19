use bevy::prelude::*;

#[derive(Component)]
pub struct CommonPlaySceneTag;

pub fn xor_u64s(values: &[u64]) -> u64 {
    values.iter().fold(0, |acc, &val| acc ^ val)
}

pub fn isometric_to_cartesian(iso_x: f32, iso_y: f32) -> Vec2 {
    Vec2::new(
        (iso_x - 2.0 * iso_y) / 2.0, // Cartesian X
        (iso_x + 2.0 * iso_y) / 2.0, // Cartesian Y
    )
}

pub fn cartesian_to_isometric(cart_x: f32, cart_y: f32) -> Vec2 {
    Vec2::new(
        cart_x + cart_y,         // X-axis in isometric space
        (cart_y - cart_x) / 2.0, // Y-axis in isometric space
    )
}

pub fn u64_to_vec3(value: u64) -> Vec3 {
    let high = (value >> 42) as f32;
    let mid = ((value >> 20) & 0x3FFFFF) as f32;
    let low = (value & 0xFFFFF) as f32;
    Vec3::new(high, mid, low)
}

pub fn vec3_to_u64(v: Vec3) -> u64 {
    ((v.x as u64) << 42) | ((v.y as u64) << 20) | (v.z as u64)
}
