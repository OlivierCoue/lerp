use std::sync::atomic::AtomicU32;

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
