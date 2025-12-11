use bevy::prelude::*;

pub mod resources;

use crate::midi::resources::{MIDISequence, Tempo};

use self::resources::Playback;

pub struct PlaybackPlugin;

impl Plugin for PlaybackPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Playback::default())
            .add_systems(Update, (update_keyboard_event, update_ticks));
    }
}

fn update_ticks(
    mut playback: ResMut<Playback>,
    tempo: Res<Tempo>,
    time: Res<Time>,
    midi_sequence: Res<MIDISequence>,
) {
    playback.add_ticks((time.delta_secs_f64() * midi_sequence.timing_unit / tempo.secs()) as u32);
}

fn update_keyboard_event(keys: Res<ButtonInput<KeyCode>>, mut playback: ResMut<Playback>) {
    if keys.just_pressed(KeyCode::Space) {
        if playback.is_playing() {
            playback.pause();
            debug!("Paused");
            debug!("{}", playback.ticks());
        } else {
            playback.play();
            debug!("Playing");
            debug!("{}", playback.ticks());
        }
    }
}
