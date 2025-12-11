use midly::{Smf, Timing};
use std::fs;

use crate::midi::resources::{MIDIEvent, MIDISequence, MIDITrack};

pub fn parse_smf_file(filename: &str) -> Result<MIDISequence, &str> {
    let raw = fs::read(filename).expect("cannot open midi file").leak();
    let smf = Smf::parse(raw).expect("cannot parse midi file");
    let timing_unit = match smf.header.timing {
        Timing::Metrical(u) => u.as_int() as f64,
        _ => return Err("SMPTE timing is not supported"),
    };
    let mut tracks = Vec::new();

    for (index, track) in smf.tracks.iter().enumerate() {
        let mut elapsed_ticks = 0;
        let mut events = Vec::new();

        for event in track {
            elapsed_ticks += event.delta.as_int();
            events.push(MIDIEvent {
                elapsed_ticks,
                kind: event.kind,
            });
        }

        tracks.push(MIDITrack {
            events,
            number: index as u8,
            delta_secs: 0.,
            next_event_index: 0,
        });
    }

    Ok(MIDISequence {
        tracks: tracks,
        timing_unit,
    })
}
