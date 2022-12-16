use std::{collections::HashMap, hash::Hash};

use bevy::prelude::*;
use meshtext::IndexedMeshText;

// TODO: add accuracy to depth cache
// TODO: purge cached entries, keep count per depth, and if it reaches zero
// TODO: actually cache meshdata

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct CacheKey {
    char: char,
    mesh_type: MeshType,
}

impl CacheKey {
    pub(crate) fn new_3d(char: char, depth: f32) -> Self {
        Self {
            char,
            mesh_type: MeshType::Mesh3d(Depth(depth)),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum MeshType {
    // Mesh2d,
    Mesh3d(Depth),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Depth(f32);

impl Hash for Depth {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let val = (self.0 * 100.) as usize;
        val.hash(state);
    }
}

// FIXME uhm, empty?
impl Eq for Depth {}

#[derive(Resource)]
pub struct MeshCache {
    pub(crate) meshes: HashMap<CacheKey, IndexedMeshText>,
}

impl Default for MeshCache {
    fn default() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }
}

unsafe impl Send for MeshCache {} // FIXME: verify soundness
unsafe impl Sync for MeshCache {} // FIXME: verify soundness
