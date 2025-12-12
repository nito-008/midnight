use bevy::prelude::*;
use std::fmt;

#[derive(Resource)]
pub struct Playback {
    ticks: u32,
    state: PlaybackState,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

impl Playback {
    pub fn new() -> Self {
        Self {
            ticks: 0,
            state: PlaybackState::Stopped,
        }
    }

    pub fn ticks(&self) -> u32 {
        self.ticks
    }

    pub fn state(&self) -> PlaybackState {
        self.state
    }

    pub fn stop(&mut self) {
        self.ticks = 0;
        self.state = PlaybackState::Stopped;
    }

    pub fn pause(&mut self) {
        self.state = PlaybackState::Paused;
    }

    pub fn play(&mut self) {
        self.state = PlaybackState::Playing;
    }

    pub fn add_ticks(&mut self, delta_ticks: u32) {
        self.ticks += delta_ticks;
    }

    pub fn is_playing(&self) -> bool {
        matches!(self.state, PlaybackState::Playing)
    }
}

impl fmt::Display for PlaybackState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlaybackState::Stopped => write!(f, "Stopped"),
            PlaybackState::Playing => write!(f, "Playing"),
            PlaybackState::Paused => write!(f, "Paused"),
        }
    }
}
