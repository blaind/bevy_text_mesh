use bevy::prelude::*;
use bevy_text_mesh::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(TextMeshPlugin)
        .add_startup_system(spawn_text.system())
        .add_startup_system(setup.system())
        .run();
}

fn spawn_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands.spawn_bundle(TextMeshBundle {
        text_mesh: TextMesh::new_with_color(
            "This text is aligned to VerticalAlign::Top and HorizontalAlign::Left with wrapping=true",
            font.clone(),
            Color::rgb(1., 1., 0.),
        ),
        transform: Transform::from_xyz(0., 1.5, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(TextMeshBundle {
        text_mesh: TextMesh {
            text: String::from("wrapping=false, overflow..."),
            style: TextMeshStyle {
                font: font.clone(),
                ..Default::default()
            },
            size: TextMeshSize {
                width: SizeUnit::NonStandard(36. * 4.),
                height: SizeUnit::NonStandard(36. * 2.),
                wrapping: false,
                ..Default::default()
            },
            ..Default::default()
        },
        transform: Transform::from_xyz(-2.0, 2.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(TextMeshBundle {
        text_mesh: TextMesh {
            text: String::from("This text is quite deep"),
            style: TextMeshStyle {
                font: font.clone(),
                color: Color::rgb(1., 0., 0.),
                ..Default::default()
            },
            size: TextMeshSize {
                width: SizeUnit::NonStandard(36. * 4.),
                height: SizeUnit::NonStandard(36. * 1.),
                wrapping: false,
                depth: Some(SizeUnit::NonStandard(144.)),
                ..Default::default()
            },
            ..Default::default()
        },
        //transform: Transform::from_xyz(-2.0, 1.5, 1.5).looking_at(Vec3::new(-7., 0., -7.), Vec3::Y),
        transform: Transform::from_xyz(-2.0, 1.5, 1.5).looking_at(Vec3::new(-7., 0., -7.), Vec3::Y),
        ..Default::default()
    });

    commands
        .spawn_bundle(TextMeshBundle {
            text_mesh: TextMesh {
                text: String::from("Custom material which is transparent"),
                style: TextMeshStyle {
                    font: font.clone(),
                    ..Default::default()
                },
                size: TextMeshSize {
                    width: SizeUnit::NonStandard(36. * 4.),
                    height: SizeUnit::NonStandard(36. * 2.),
                    wrapping: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_xyz(-1.5, 0.5, 1.3),
            ..Default::default()
        })
        .insert(materials.add(StandardMaterial {
            base_color: Color::rgba(1., 0., 0., 0.4),
            ..Default::default()
        }))
        .insert(Visible {
            is_transparent: true,
            ..Default::default()
        });
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.5, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
