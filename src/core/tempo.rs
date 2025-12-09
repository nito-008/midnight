use bevy::prelude::*;

#[derive(Resource)]
pub struct Tempo {
    secs: f64,
}

impl Tempo {
    pub fn from_secs(secs: f64) -> Self {
        Self { secs }
    }

    pub fn from_bpm(bpm: f64) -> Self {
        Self { secs: 60.0 / bpm }
    }

    /// Returns the duration of a quarter note.
    pub fn secs(&self) -> f64 {
        self.secs
    }

    pub fn set_secs(&mut self, secs: f64) {
        self.secs = secs;
        println!("Tempo(secs): {}", secs);
        println!("Tempo(bpm): {}", self.bpm());
    }

    pub fn bpm(&self) -> f64 {
        60.0 / self.secs
    }

    pub fn set_bpm(&mut self, bpm: f64) {
        self.secs = 60.0 / bpm;
    }
}

fn get_delta_secs(timing_unit: f64, delta_time: f64, tempo_secs: f64) -> f64 {
    let delta = delta_time / timing_unit * tempo_secs;
    delta
}
