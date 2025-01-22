// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};

// pub fn hash_u64s(values: &[u64]) -> u64 {
//     let mut hasher = DefaultHasher::new();
//     for value in values {
//         value.hash(&mut hasher);
//     }
//     hasher.finish()
// }

use bevy::prelude::Component;

#[derive(Component)]
pub struct CommonPlaySceneTag;

pub fn xor_u64s(values: &[u64]) -> u64 {
    values.iter().fold(0, |acc, &val| acc ^ val)
}
