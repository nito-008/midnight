use bevy::prelude::*;
use midly::TrackEventKind;

pub struct MIDIEvent {
    elapsed_ticks: u32,
    kind: TrackEventKind<'static>,
}

impl MIDIEvent {
    pub fn new(elapsed_ticks: u32, kind: TrackEventKind<'static>) -> Self {
        Self {
            elapsed_ticks,
            kind,
        }
    }
}

#[derive(Resource)]
pub struct MIDITracks(Vec<Vec<MIDIEvent>>);

impl MIDITracks {
    pub fn new(midi_tracks: Vec<Vec<MIDIEvent>>) -> Self {
        Self(midi_tracks)
    }
}
