pub mod components;

use std::collections::HashMap;

use bevy::prelude::*;
use midly::{MidiMessage, TrackEventKind};

use crate::{midi::resources::MIDISequence, visualization::components::Note};

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_notes);
    }
}

fn setup_notes(mut commands: Commands, midi_sequence: Res<MIDISequence>) {
    for track in &midi_sequence.tracks {
        let mut active_notes = HashMap::new();

        for event in &track.events {
            if let TrackEventKind::Midi { channel, message } = event.kind {
                match message {
                    MidiMessage::NoteOn { key, .. } => {
                        active_notes.insert(key, event.elapsed_ticks);
                    }
                    MidiMessage::NoteOff { key, vel } => {
                        if let Some(start_ticks) = active_notes.remove(&key) {
                            commands.spawn(Note {
                                channel,
                                key,
                                vel,
                                length: event.elapsed_ticks - start_ticks,
                                start_ticks,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn 