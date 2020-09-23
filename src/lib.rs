mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone)]
pub struct RollNote {
    on: bool,
    trig_chance: u8, // percent
    velocity: u8, // 0-100
}

impl RollNote {
    pub fn new(on: bool, trig_chance: u8, velocity: u8) -> RollNote {
        RollNote {
            on,
            trig_chance,
            velocity,
        }
    }

    pub fn toggle(&mut self) {
        self.on = !self.on;
    }
}


#[derive(Clone)]
pub struct RollRow {
    row: Vec<RollNote>,
}

#[wasm_bindgen]
pub struct WebSeq {
    tempo: u8,
    bars: u8, // 2, 4, 8, 16, 32
    notes_per_bar: u8, // piano roll
    roll: Vec<RollRow>,
    step_index: u8,
    rows: u8,
}

#[wasm_bindgen]
impl WebSeq {
    pub fn new(tempo: u8, bars: u8, notes_per_bar: u8, rows: u8) -> WebSeq {
        
        let roll_note = RollNote::new(true, 100, 100);
        
        // How many notes per piano roll row
        let steps: usize = (notes_per_bar * bars).into();
        // Init one row
        let row = RollRow {
            row: vec![roll_note; steps],
        };
        
        WebSeq {
            tempo: tempo,
            bars: bars,
            notes_per_bar: notes_per_bar,
            roll: vec![row; usize::from(rows)],
            step_index: 0,
            rows,
        }
    }


    fn get_steps(&self) -> u8 {
        self.bars * self.notes_per_bar
    }

    pub fn step(&mut self) {
        let row_notes = self.get_steps();
        if self.step_index == row_notes {
            self.step_index = 0;
        }
        self.step_index += 1;
    }

    // Get set
    pub fn get_tempo(&self) -> u8 {
        self.tempo
    }

    pub fn set_tempo(&mut self, tempo: u8) {
        self.tempo = tempo;
    }

    pub fn get_step_index(&self) -> u8 {
        self.step_index
    }

    pub fn render(&self) -> String {
        format!("Ok")
    }

}