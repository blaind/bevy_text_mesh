use std::time::Duration;

use bevy::{prelude::*, render::camera::Camera};
use bevy_text_mesh::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(TextMeshPlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_text_mesh)
        .add_system(update_text_mesh)
        .add_system(rotate_camera)
        .run();
}

fn setup_text_mesh(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraMono-Medium.ttf#mesh");

    commands.spawn(TextMeshBundle {
        text_mesh: TextMesh {
            text: String::from("Time since startup"),
            style: TextMeshStyle {
                font: font.clone(),
                font_size: SizeUnit::NonStandard(9.),
                color: Color::rgb(0.0, 0.0, 0.0),
                ..Default::default()
            },
            size: TextMeshSize {
                ..Default::default()
            },
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-1., 1.75, 0.),
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn(TextMeshBundle {
            text_mesh: TextMesh {
                text: String::from("0"),
                style: TextMeshStyle {
                    font: font.clone(),
                    font_size: SizeUnit::NonStandard(36.),
                    color: Color::rgb(0.0, 1.0, 0.0),
                    mesh_quality: Quality::Custom(128),
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-1., 1.3, 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EngineTime);

    commands.insert_resource(UpdateTimer {
        timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
    });
}

#[derive(Resource)]
struct UpdateTimer {
    timer: Timer,
}

#[derive(Component)]
struct EngineTime;

fn update_text_mesh(
    time: Res<Time>,
    mut text_meshes: Query<&mut TextMesh, With<EngineTime>>,
    mut timer: ResMut<UpdateTimer>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        for mut text_mesh in text_meshes.iter_mut() {
            let updated_text = String::from(format!("Time = {:.3}", time.elapsed_seconds_f64()));

            if text_mesh.text != updated_text {
                text_mesh.text = updated_text;
            }
        }
    }
}

fn rotate_camera(mut camera: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    for mut camera in camera.iter_mut() {
        let angle = time.elapsed_seconds_f64() as f32 / 2. + 1.55 * std::f32::consts::PI;

        let distance = 3.5;

        camera.translation = Vec3::new(
            angle.sin() as f32 * distance,
            camera.translation.y,
            angle.cos() as f32 * distance,
        );

        *camera = camera.looking_at(Vec3::new(0.0, 1.5, 0.), Vec3::Y);
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
