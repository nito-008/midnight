use bevy::prelude::*;

pub struct MainWindowPlugin;

impl Plugin for MainWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_window);
    }
}

fn setup_window(mut commands: Commands) {
    commands.spawn(Camera2d);
}
