use bevy::prelude::*;
use meshtext::{Glyph, MeshGenerator};

use crate::{
    mesh_cache::{CacheKey, MeshCache},
    text_mesh::{FontStyle, TextMesh},
};

pub(crate) struct MeshData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub uvs: Vec<[f32; 2]>,
}

// FIXME: add validator, that validates all .unwrap's() at addition time
// now crashes might occur
//
// TODO: optimization possibility - take char mesh up to modified char
// from the existing mesh
pub(crate) fn generate_text_mesh(
    text_mesh: &TextMesh,
    font: &mut MeshGenerator,
    cache: Option<&mut MeshCache>,
) -> MeshData {
    trace!("Generate text mesh: {:?}", text_mesh.text);

    let mut internal_cache;

    let cache = match cache {
        Some(cache) => cache,
        None => {
            internal_cache = Some(MeshCache::default());
            internal_cache.as_mut().unwrap()
        }
    };

    let mut vertices_offset: usize = 0;

    let depth = 0.08;

    let text = if text_mesh.style.font_style.contains(FontStyle::UPPERCASE) {
        text_mesh.text.to_uppercase()
    } else if text_mesh.style.font_style.contains(FontStyle::LOWERCASE) {
        text_mesh.text.to_lowercase()
    } else {
        text_mesh.text.clone() // TODO performance - extra allocation
    };

    let scalar = match text_mesh.style.font_size.as_scalar() {
        Some(scalar) => scalar,
        None => todo!("Font automatic sizing has not been implemented yet"),
    };

    let mut scaled_offset = Vec2::ZERO;
    let mut scaled_row_y_max_height = 0.;

    //TODO: use the input in text_mesh to generate the Mat4 vector to be used for transformation when generating the text.
    use meshtext::TextSection;
    let sectionmesh: meshtext::IndexedMeshText = font.generate_section(&text, false, None).unwrap();

    let indices = sectionmesh.indices.clone();
    let vertices: Vec<[f32; 3]> = sectionmesh
        .vertices
        .chunks(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect();
    let normals: Vec<[f32; 3]> = sectionmesh
        .vertices
        .chunks(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect();

    let uvs = vertices.iter().map(|_vert| [0., 1.]).collect::<Vec<_>>();

    MeshData {
        vertices,
        normals,
        indices,
        uvs,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mesh_data_generator::generate_text_mesh, text_mesh::TextMesh, SizeUnit, TextMeshSize,
        TextMeshStyle,
    };

    use super::*;

    pub(crate) fn get_font_bytes() -> Vec<u8> {
        std::fs::read("./assets/fonts/FiraMono-Medium.ttf").unwrap()
    }

    #[test]
    fn test_generate_mesh() {
        let mut mesh_cache = MeshCache::default();
        let bytes = get_font_bytes().leak();
        let mut font = MeshGenerator::new(bytes);

        let text_mesh = TextMesh {
            text: "hello world!".to_string(),
            size: TextMeshSize {
                width: SizeUnit::NonStandard(36. * 2.),
                height: SizeUnit::NonStandard(36. * 5.),
                ..Default::default()
            },
            style: TextMeshStyle {
                font_size: SizeUnit::NonStandard(18.),
                ..Default::default()
            },
            ..Default::default()
        };

        let _ = generate_text_mesh(&text_mesh, &mut font, Some(&mut mesh_cache));
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_get_glyph_cached(b: &mut Bencher) {
        let mut mesh_cache = MeshCache::default();
        let mut font = ttf2mesh::TTFFile::from_buffer_vec(tests::get_font_bytes()).unwrap();

        let text_mesh = TextMesh::new_no_font("hello world!".to_string());
        let _ = generate_text_mesh(&text_mesh, &mut font, Some(&mut mesh_cache));

        b.iter(|| {
            let _ = generate_text_mesh(&text_mesh, &mut font, Some(&mut mesh_cache));
        });
    }

    #[bench]
    fn bench_get_glyph_no_cache(b: &mut Bencher) {
        let mut font = ttf2mesh::TTFFile::from_buffer_vec(tests::get_font_bytes()).unwrap();
        let text_mesh = TextMesh::new_no_font("hello world!".to_string());

        b.iter(|| {
            let _ = generate_text_mesh(&text_mesh, &mut font, None);
        });
    }
}
