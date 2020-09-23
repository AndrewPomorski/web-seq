mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Clone)]
pub struct RollNote {
    on: bool,
    trig_chance: u8, // percent
    velocity: u8, // 0-100
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct RollRow {
    row: Vec<RollNote>,
}

#[wasm_bindgen]
pub struct WebSeq {
    tempo: u8,
    time: u8, // 4, 8, 16, 32, 64
    bars: u8, // 2, 4, 8, 16, 32
    notes_per_bar: u8, // piano roll
    roll: Vec<RollRow>,
    step_index: u8,
}

impl WebSeq {
    pub fn new(tempo: u8, time: u8, bars: u8, notes_per_bar: u8, mut roll: Vec<RollRow>, step_index: u8) -> WebSeq {
        let roll_note = RollNote {
            on: true,
            trig_chance: 100,
            velocity: 100,
        };
        let row = RollRow {
            row: vec![roll_note; 4],
        };
        roll = vec![row; 4];
        WebSeq {
            tempo: tempo,
            time: time,
            bars: bars,
            notes_per_bar: notes_per_bar,
            roll: roll,
            step_index: step_index,
        }
    }

    pub fn get_tempo(&self) -> &u8 {
        &self.tempo
    }

    pub fn set_tempo(&mut self, tempo: u8) {
        self.tempo = tempo;
    }

}