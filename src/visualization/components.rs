use bevy::prelude::*;
use midly::num::{u4, u7};

#[derive(Component)]
pub struct Note {
    pub channel: u4,
    pub key: u7,
    pub vel: u7,
    pub length: u32,
    pub start_ticks: u32,
}
