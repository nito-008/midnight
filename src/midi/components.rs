use bevy::prelude::*;

use midly::num::{u4, u7};

#[derive(Component)]
pub struct Note {
    pub ch: u4,
    pub key: u7,
    pub vel: u7,
    pub length: f64,
    pub start_time: f64,
}
