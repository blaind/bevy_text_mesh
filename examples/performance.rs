use std::time::Duration;

use bevy::{
    diagnostic::{
        Diagnostic, DiagnosticId, Diagnostics, DiagnosticsStore, FrameTimeDiagnosticsPlugin,
        LogDiagnosticsPlugin, RegisterDiagnostic,
    },
    prelude::*,
    render::camera::Camera,
};
use bevy_text_mesh::prelude::*;

use rand::prelude::*;

// NOTE! Custom (unlit) material used

// tessellation quality
const MESH_QUALITY: Quality = Quality::Low;

// how often new texts are spawned
const TEXT_SPAWN_INTERVAL: u64 = 125;

// how often spawned texts are updated
const TEXT_UPDATE_INTERVAL_MS: u64 = 1;

// initial wait time before starting spawn
const INITIAL_WAIT_MS: u64 = 500;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            DefaultPlugins,
            TextMeshPlugin,
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ))
        .register_diagnostic(Diagnostic::new(TEXT_MESH_UPDATES, "text_mesh_updates", 20))
        .add_systems(Startup, (setup, setup_text_mesh))
        .add_systems(Update, (spawn_meshes, update_text_mesh, rotate_camera))
        .add_systems(PostUpdate, update_frame_rate)
        .run();
}

#[derive(Resource)]
struct SceneState {
    font: Handle<TextMeshFont>,
    material: Handle<StandardMaterial>,
    text_count: usize,
    text_update_count: usize,
}

#[derive(Resource)]
struct UpdateTimer {
    spawn_new_text_timer: Timer,
    fps_update_timer: Timer,
    text_update_timer: Timer,
}

#[derive(Component)]
struct EngineTime;

#[derive(Component)]
struct FPS;

#[derive(Component)]
struct TextCount;

pub const TEXT_MESH_UPDATES: DiagnosticId =
    DiagnosticId::from_u128(1082410928401928501928509128509125);

fn setup_text_mesh(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let state = SceneState {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        text_count: 0,
        text_update_count: 0,
        material: materials.add(StandardMaterial {
            base_color: Color::BLACK,
            unlit: true,
            ..Default::default()
        }),
    };

    commands
        .spawn(TextMeshBundle {
            text_mesh: TextMesh {
                text: String::from("FPS"),
                style: TextMeshStyle {
                    font: state.font.clone(),
                    color: Color::rgb(0., 1., 0.),
                    font_size: SizeUnit::NonStandard(48.),
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 2.5, 0.),
            ..Default::default()
        })
        .insert(FPS);

    commands
        .spawn(TextMeshBundle {
            text_mesh: TextMesh {
                text: String::from("Text count"),
                style: TextMeshStyle {
                    font: state.font.clone(),
                    font_size: SizeUnit::NonStandard(18.),
                    color: Color::rgb(1., 1., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 3., 0.),
            ..Default::default()
        })
        .insert(TextCount);

    commands.insert_resource(state);

    commands.insert_resource(UpdateTimer {
        spawn_new_text_timer: Timer::new(Duration::from_millis(INITIAL_WAIT_MS), TimerMode::Once),
        text_update_timer: Timer::new(
            Duration::from_millis(TEXT_UPDATE_INTERVAL_MS),
            TimerMode::Repeating,
        ),

        // how often FPS text is updated
        fps_update_timer: Timer::new(Duration::from_millis(150), TimerMode::Repeating),
    });
}

fn spawn_meshes(
    mut commands: Commands,
    mut state: ResMut<SceneState>,
    time: Res<Time>,
    mut timer: ResMut<UpdateTimer>,
) {
    if timer
        .spawn_new_text_timer
        .tick(time.delta())
        .just_finished()
    {
        timer.spawn_new_text_timer =
            Timer::new(Duration::from_millis(TEXT_SPAWN_INTERVAL), TimerMode::Once);

        let mut rng = rand::thread_rng(); // how performant is this?

        let transform = Transform {
            translation: Vec3::new(
                rng.gen_range(-1.0..1.0) * 2.0,
                rng.gen::<f32>() * 2.0,
                rng.gen_range(-1.0..1.0) * 2.0,
            ),
            scale: Vec3::ONE * (1. - rng.gen::<f32>() * 0.8) * 0.5,
            ..Default::default()
        }
        .looking_at(
            Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()),
            Vec3::Y,
        );

        commands
            .spawn(TextMeshBundle {
                text_mesh: TextMesh {
                    text: String::from(""),
                    style: TextMeshStyle {
                        font: state.font.clone(),
                        mesh_quality: MESH_QUALITY,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                transform,
                ..Default::default()
            })
            .insert(EngineTime)
            .insert(state.material.clone());

        state.text_count += 1;
    }
}

fn update_text_mesh(
    mut diagnostics: Diagnostics,
    mut text_meshes: Query<&mut TextMesh, With<EngineTime>>,
    time: Res<Time>,
    mut timer: ResMut<UpdateTimer>,
    mut state: ResMut<SceneState>,
) {
    let mut update_count = 0;
    if timer.text_update_timer.tick(time.delta()).just_finished() {
        for mut text_mesh in text_meshes.iter_mut() {
            let updated_text = String::from(format!("Time = {:.3}", time.elapsed_seconds_f64()));

            if text_mesh.text != updated_text {
                text_mesh.text = updated_text;
                update_count += 1;
            }
        }
    }

    state.text_update_count += update_count;
    diagnostics.add_measurement(TEXT_MESH_UPDATES, || state.text_update_count as f64);
}

fn rotate_camera(mut camera: Query<&mut Transform, With<Camera>>, time: Res<Time>) {
    for mut camera in camera.iter_mut() {
        let angle = time.elapsed_seconds_f64() as f32 / 2. + 1.55 * std::f32::consts::PI;

        let distance = 6.5;

        camera.translation = Vec3::new(
            angle.sin() as f32 * distance,
            camera.translation.y,
            angle.cos() as f32 * distance,
        );

        *camera = camera.looking_at(Vec3::new(0.0, 1.5, 0.), Vec3::Y);
    }
}

fn update_frame_rate(
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
    mut timer: ResMut<UpdateTimer>,
    mut fps_text: Query<(Entity, &mut TextMesh, Option<&FPS>), Or<(With<FPS>, With<TextCount>)>>,
    camera_entity: Query<Entity, With<Camera>>,
    mut transform_query: Query<&mut Transform>,
    state: Res<SceneState>,
) {
    for (text_mesh_entity, mut text_mesh, fps) in fps_text.iter_mut() {
        if timer.fps_update_timer.tick(time.delta()).just_finished() {
            if fps.is_some() {
                let fps = diagnostics
                    .get_measurement(FrameTimeDiagnosticsPlugin::FPS)
                    .unwrap();

                text_mesh.text = format!("FPS={}", fps.value.round() as usize);
            } else {
                text_mesh.text = format!("{} text items", state.text_count);
            }
        }

        let camera_entity = camera_entity.iter().next().unwrap();
        let camera_transform = transform_query.get_mut(camera_entity).unwrap().clone();
        let mut transform = transform_query.get_mut(text_mesh_entity).unwrap();

        // eh - why negative?
        *transform = transform.looking_at(-camera_transform.translation, Vec3::Y);
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
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
