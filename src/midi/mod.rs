pub mod components;
pub mod midi_parser;
pub mod resources;

use bevy::prelude::*;
use midly::{MetaMessage, MidiMessage, TrackEventKind};

use crate::{
    midi::resources::{MIDIEvent, MIDISequence, Tempo},
    playback::resources::Playback,
};

pub struct MidiPlugin;

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut App) {
        let midi_sequence =
            midi_parser::parse_smf_file("test.mid").expect("failed to parse midi file");
        let tempo = Tempo::from_bpm(120.);
        app.insert_resource(midi_sequence)
            .insert_resource(tempo)
            .add_systems(Update, update_midi_events);
    }
}

fn update_midi_events(
    mut midi_sequence: ResMut<MIDISequence>,
    time: Res<Time>,
    mut tempo: ResMut<Tempo>,
    playback: Res<Playback>,
) {
    if !playback.is_playing() {
        return;
    }

    for track in &mut midi_sequence.tracks {
        let events = &mut track.events;
        track.delta_secs += time.delta_secs_f64();

        loop {
            if track.next_event_index >= events.len() as u32 {
                break;
            }

            let event = &events[track.next_event_index as usize];

            if event.elapsed_ticks <= playback.ticks() {
                on_track_event(event, &mut tempo);
                track.next_event_index += 1;
            } else {
                break;
            }
        }
    }
}

fn on_track_event(event: &MIDIEvent, tempo: &mut ResMut<Tempo>) {
    match event.kind {
        TrackEventKind::Midi { channel, message } => {
            debug!("Channel {} : ", channel);
            match message {
                MidiMessage::NoteOn { key, vel } => {
                    debug!("ON {} Vel {}", key, vel);
                }
                MidiMessage::NoteOff { key, vel } => {
                    debug!("OFF {} Vel {}", key, vel);
                }
                MidiMessage::ProgramChange { program } => {
                    debug!("Program Change");
                }
                MidiMessage::Aftertouch { key, vel } => {
                    debug!("Aftertouch {}", key);
                }
                MidiMessage::ChannelAftertouch { vel } => {
                    debug!("ChannelAftertouch");
                }
                MidiMessage::Controller { controller, value } => {
                    debug!("Controller {} {}", controller, value);
                }
                MidiMessage::PitchBend { bend } => {
                    debug!("PitchBend");
                }
            }
        }
        TrackEventKind::Meta(meta_message) => match meta_message {
            MetaMessage::Tempo(tempo_micros) => {
                tempo.set_secs(tempo_micros.as_int() as f64 / 1000000.0);
            }
            MetaMessage::TrackName(track_name) => {}
            _ => {
                debug!("MetaMessage");
            }
        },
        _ => {}
    }
}
