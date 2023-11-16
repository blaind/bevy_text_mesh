#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
extern crate bitflags;

use bevy::prelude::*;

mod font_loader;
mod mesh_cache;
mod mesh_data_generator;
mod mesh_system;
mod text_mesh;

pub mod prelude {
    pub use crate::font_loader::TextMeshFont;
    pub use crate::text_mesh::*;
    pub use crate::TextMeshPlugin;
    pub use glyph_brush_layout::{HorizontalAlign, VerticalAlign};
}

use font_loader::FontLoader;
use mesh_cache::MeshCache;
pub use prelude::*;

pub struct TextMeshPlugin;

impl Plugin for TextMeshPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(FontLoader)
            .init_asset::<font_loader::TextMeshFont>()
            .add_systems(Update, (mesh_system::text_mesh, mesh_system::font_loaded))
            .insert_resource(MeshCache::default())
            .init_asset_loader::<font_loader::FontLoader>();
    }
}
