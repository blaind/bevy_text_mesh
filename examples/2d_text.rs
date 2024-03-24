use bevy::prelude::*;
use bevy_text_mesh::TextMeshPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextMeshPlugin)) // TextMeshPlugin for interop check
        .add_systems(Startup, setup)
        .add_systems(Update, animate_rotation)
        .run();
}

#[derive(Component)]
struct AnimateRotation;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = JustifyText::Center;

    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(Text2dBundle {
            text: Text::from_section("standard 2d text works too", text_style.clone())
                .with_justify(text_alignment),
            ..default()
        })
        .insert(AnimateRotation);
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds_f64().cos() as f32);
    }
}
