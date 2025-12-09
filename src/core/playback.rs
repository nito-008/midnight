use std::fmt;

use bevy::prelude::*;

#[derive(Resource)]
struct Playback {
    ticks: u32,
    state: PlaybackState,
}

#[derive(Clone, Copy, PartialEq)]
enum PlaybackState {
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
        match self.state {
            PlaybackState::Playing => true,
            _ => false,
        }
    }
}

impl Default for Playback {
    fn default() -> Self {
        Self {
            ticks: 0,
            state: PlaybackState::Stopped,
        }
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

pub struct PlaybackPlugin;

impl Plugin for PlaybackPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Playback::default())
            .add_systems(Update, update_playback_keyboard_event);
    }
}

fn update_playback_keyboard_event(keys: Res<ButtonInput<KeyCode>>, mut playback: ResMut<Playback>) {
    if keys.just_pressed(KeyCode::Space) {
        if playback.is_playing() {
            playback.stop();
        } else {
            playback.play();
        }
    }
}
