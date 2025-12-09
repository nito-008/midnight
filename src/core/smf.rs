use std::fs;

use super::midi_event::{MIDIEvent, MIDITracks};
use bevy::ecs::resource::Resource;
use midly::{Smf, Timing};

#[derive(Resource)]
pub struct ParsedSmfData {
    midi_tracks: MIDITracks,
    timing_unit: f64,
}

impl ParsedSmfData {
    fn new(midi_tracks: MIDITracks, timing_unit: f64) -> ParsedSmfData {
        ParsedSmfData {
            midi_tracks,
            timing_unit,
        }
    }
}

pub fn parse_smf_file(file_path: &str) -> Result<ParsedSmfData, String> {
    let raw = fs::read(file_path).expect("cannot open midi file").leak();
    let smf = Smf::parse(raw).expect("cannot parse midi file");
    let timing_unit = match smf.header.timing {
        Timing::Metrical(u) => u.as_int() as f64,
        _ => return Err("SMPTE timing is not supported".to_string()),
    };
    let mut midi_tracks = Vec::new();

    for track in smf.tracks {
        let mut elapsed_ticks = 0;
        let mut midi_track = Vec::new();

        for event in track {
            elapsed_ticks += event.delta.as_int();
            midi_track.push(MIDIEvent::new(elapsed_ticks, event.kind));
        }

        midi_tracks.push(midi_track);
    }

    Ok(ParsedSmfData::new(
        MIDITracks::new(midi_tracks),
        timing_unit,
    ))
}
