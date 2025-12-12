mod midi;
mod playback;
mod visualization;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;

use crate::midi::MidiPlugin;
use crate::playback::PlaybackPlugin;
use crate::visualization::VisualizationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_plugins(TweeningPlugin)
        .add_plugins(MidiPlugin)
        .add_plugins(PlaybackPlugin)
        .add_plugins(VisualizationPlugin)
        .run();
}
