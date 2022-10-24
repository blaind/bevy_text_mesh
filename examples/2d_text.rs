use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_rotation)
        .run();
}

#[derive(Component)]
struct AnimateRotation;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf#mesh");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;

    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("standard 2d text works too", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        })
        .insert(AnimateRotation);
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.seconds_since_startup().cos() as f32);
    }
}
