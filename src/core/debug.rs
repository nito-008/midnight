use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{WindowRef, WindowResolution},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_debug_window, setup_debug_text).chain());
    }
}

#[derive(Component)]
pub struct DebugWindowCamera;

fn setup_debug_window(mut commands: Commands) {
    let debug_window = commands
        .spawn(Window {
            title: "Debug Window".to_owned(),
            resolution: WindowResolution::new(800.0, 600.0),
            ..default()
        })
        .id();

    commands.spawn((
        Camera2d,
        Camera {
            target: RenderTarget::Window(WindowRef::Entity(debug_window)),
            ..default()
        },
        DebugWindowCamera,
    ));
}

#[derive(Component)]
pub struct DebugText;

fn setup_debug_text(mut commands: Commands, query: Query<Entity, With<DebugWindowCamera>>) {
    let debug_window_camera = query.single().unwrap();

    let debug_text = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                row_gap: Val::Px(5.0),
                ..default()
            },
            UiTargetCamera(debug_window_camera),
        ))
        .id();

    for i in 0..3 {
        let child = commands.spawn((Text::new("Loading..."), DebugText)).id();
        commands.entity(debug_text).add_children(&[child]);
    }
}

fn update_debug_text(mut commands: Commands, query: Query<&mut Text, With<DebugText>>) {}
