use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};

use crate::{
    font_loader::TextMeshFont, mesh_cache::MeshCache, mesh_data_generator::generate_text_mesh,
};
use crate::{mesh_data_generator::MeshData, text_mesh::TextMesh};

pub(crate) fn text_mesh(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fonts: ResMut<Assets<TextMeshFont>>,
    text_meshes: Query<
        (
            Entity,
            &Transform,
            &GlobalTransform,
            Option<&Handle<StandardMaterial>>,
            &TextMesh,
            Option<&Handle<Mesh>>,
            &TextMeshState,
            Option<&Visible>,
        ),
        Or<(Changed<TextMesh>, Changed<TextMeshState>)>,
    >,
    mut cache: ResMut<MeshCache>,
) {
    // per-text-mesh system. Triggered only if the TextMesh or TextMeshState change
    // = user changes text properties, or if/when the font is loaded
    // the initial render might happen before font has loaded - hence need to trigger after font load
    //
    // TODO: performance could be improved by using text_meshes.par_for_each
    // but that'd require cache to be cloneable.
    // maybe using channels could work, e.g. pre-generate sprites to cache,
    // then parallel execute each mesh generation and send results to channels
    // and finally run commands/meshes additions sequentially from channel results
    // --> requires large amount of work, performance not yet bottleneck,
    // implement in future, if needed

    // TODO: performance - split to mesh-update and mesh-create systems?

    for text_mesh in text_meshes.iter() {
        let (entity, transform, global_transform, material, text_mesh, mesh, _state, visible) =
            text_mesh;

        let font = match fonts.get_mut(&text_mesh.style.font) {
            Some(font) => font,
            None => continue, // should not reach here ever
        };

        let ttf2_mesh = generate_text_mesh(&text_mesh, &mut font.ttf_font, Some(&mut cache));

        match mesh {
            Some(mesh) => {
                let mesh = match meshes.get_mut(mesh.clone()) {
                    Some(mesh) => mesh,
                    None => continue,
                };

                apply_mesh(ttf2_mesh, mesh);

                // TODO: handle color updates
            }
            None => {
                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

                apply_mesh(ttf2_mesh, &mut mesh);

                let mut bundle = commands.entity(entity);

                bundle.insert_bundle(PbrBundle {
                    mesh: meshes.add(mesh),
                    material: material.map(|m| m.clone()).unwrap_or_else(|| {
                        materials.add(StandardMaterial {
                            base_color: text_mesh.style.color,
                            ..Default::default()
                        })
                    }),
                    transform: transform.clone(),
                    global_transform: global_transform.clone(),
                    visible: visible.map(|v| v.clone()).unwrap_or(Default::default()),
                    ..Default::default()
                });

                if 1 < 10 {
                    bundle.with_children(|cmd| {
                        cmd.spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Box::new(
                                text_mesh.size.width.as_scalar().unwrap(),
                                text_mesh.size.height.as_scalar().unwrap(),
                                text_mesh.size.width.as_scalar().unwrap() * 0.01,
                            ))),
                            material: materials.add(Color::rgba(0.5, 0.5, 0.5, 0.3).into()),
                            transform: Transform::from_xyz(0., 0., -0.0001),
                            visible: Visible {
                                is_transparent: true,
                                ..Default::default()
                            },
                            ..Default::default()
                        });

                        cmd.spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube::new(0.05))),
                            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                            transform: Transform::from_xyz(0., 0., 0.),
                            ..Default::default()
                        });
                    });
                }
            }
        }
    }
}

pub(crate) fn font_loaded(
    mut events: EventReader<AssetEvent<TextMeshFont>>,
    mut query: Query<(&mut TextMeshState, &TextMesh)>,
) {
    // FIXME: this event system is triggered any time a new text is rendered
    // by AssetEvent::Modified caused by font.get_mut(). Improve performance?

    for event in events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                for (mut state, text_mesh) in query.iter_mut() {
                    if handle == &text_mesh.style.font {
                        state.font_loaded = Some(true);
                    }
                }
            }
            AssetEvent::Removed { handle } => {
                // why would this happen? handling anyway
                for (mut state, text_mesh) in query.iter_mut() {
                    if handle == &text_mesh.style.font {
                        state.font_loaded = Some(false);
                    }
                }
            }
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct TextMeshState {
    // this state matters only when the fonts have not been loaded yet
    // will be None for text bundles spawned when fonts have are already loaded
    font_loaded: Option<bool>,
}

impl Default for TextMeshState {
    fn default() -> Self {
        Self { font_loaded: None }
    }
}

fn apply_mesh(mesh_data: MeshData, mesh: &mut Mesh) {
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, mesh_data.vertices);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_data.normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, mesh_data.uvs);
    mesh.set_indices(Some(Indices::U32(mesh_data.indices)));
}
