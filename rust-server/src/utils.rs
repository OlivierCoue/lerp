use std::{sync::atomic::AtomicU32, time::SystemTime};

use rust_common::proto::data::Point;
use serde::{Deserialize, Serialize};

use std::sync::atomic::{AtomicUsize, Ordering};

static GAME_TIME: AtomicU32 = AtomicU32::new(0);

pub fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn get_game_time() -> u32 {
    GAME_TIME.load(Ordering::Relaxed)
}

pub fn inc_game_time_millis(v: u32) -> u32 {
    GAME_TIME.fetch_add(v, Ordering::Relaxed)
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

impl Coord {
    pub fn to_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
            ..Default::default()
        }
    }
}

pub fn get_timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
