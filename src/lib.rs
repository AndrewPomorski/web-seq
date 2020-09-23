mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct RollNote {
    on: bool,
    percent_trig: u8,
    velocity: u8, // 0-100%
}

#[wasm_bindgen]
pub struct WebSeq {
    time: u8, // 4, 8, 16, 32, 64
    bars: u8, // 2, 4, 8, 16, 32
    notes: u8, // piano roll
    piano_roll_notes: Vec<Vec<RollNote>>,
}