use bevy::prelude::*;
use ttf2mesh::{TTFFile, Value};

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
    font: &mut TTFFile,
    cache: Option<&mut MeshCache>,
) -> MeshData {
    trace!("Generate text mesh: {:?}", text_mesh.text);

    let mut internal_cache;

    let cache = match cache {
        Some(cache) => cache,
        None => {
            internal_cache = Some(MeshCache::default());
            internal_cache
                .as_mut()
                .expect("could not get internal_cache")
        }
    };

    // TODO performance: pre-allocate capacity
    let mut vertices = Vec::new(); //with_capacity(4308); // TODO: allocate opportunistically
    let mut normals = Vec::new(); //with_capacity(4308); // TODO: allocate opportunistically
    let mut indices = Vec::new(); //with_capacity(8520);

    let mut vertices_offset: usize = 0;

    let depth = text_mesh
        .size
        .depth
        .as_ref()
        .map(|d| d.as_scalar().unwrap())
        .unwrap_or(0.15);

    let text = if text_mesh.style.font_style.contains(FontStyle::UPPERCASE) {
        text_mesh.text.to_uppercase()
    } else if text_mesh.style.font_style.contains(FontStyle::LOWERCASE) {
        text_mesh.text.to_lowercase()
    } else {
        text_mesh.text.clone() // TODO performance - extra allocation
    };

    let scale_factor = match text_mesh.style.font_size.as_scalar() {
        Some(scalar) => scalar,
        None => todo!("Font automatic sizing has not been implemented yet"),
    };

    let spacing = Vec2::new(0.08, 0.1) * scale_factor;

    let line_start = -text_mesh.size.width.as_scalar().unwrap() / 2.0;
    let line_end = text_mesh.size.width.as_scalar().unwrap() / 2.0;

    let vertical_max_y = text_mesh.size.height.as_scalar().unwrap() / 2.0;

    let mut scaled_offset = Vec2::new(line_start, vertical_max_y - 1.0 * scale_factor);

    let mut scaled_row_y_max_height = 0.;

    //println!("scalar={}, spacing={}", scalar, spacing);
    for char in text.chars() {
        //println!("{} offset={}", char, scaled_offset);
        if char == ' ' {
            scaled_offset.x += 0.2 * scale_factor + spacing.x;
            continue;
        }

        let key = CacheKey::new_3d(char, depth);

        let mesh = match cache.meshes.get(&key) {
            Some(mesh) => mesh,
            None => {
                let glyph = font.glyph_from_char(char);

                let mut glyph = match glyph {
                    Ok(glyph) => glyph,
                    Err(_) => {
                        println!("Glyph {} not found", char);
                        font.glyph_from_char('?')
                            .expect("could not find fallback glyph icon")
                    }
                };

                let mesh = match &text_mesh.size.depth {
                    Some(unit) => glyph
                        .to_3d_mesh(
                            text_mesh.style.mesh_quality,
                            unit.as_scalar().expect("unit.as_scalar() failed"),
                        )
                        .expect("TTFFont to glyph failed"),
                    None => todo!("2d glyphs are not implemented yet. Define depth"),
                };

                cache.meshes.insert(key.clone(), mesh);
                cache.meshes.get(&key).unwrap()
            }
        };

        let (mut xmin, mut xmax) = (f32::MAX, f32::MIN);
        let (mut ymin, mut ymax) = (f32::MAX, f32::MIN);
        for vertex in mesh.iter_vertices() {
            let (x, y, _z) = vertex.val();
            // optimization possibility: calculate per-glyph min/max when caching
            if x < xmin {
                xmin = x;
            }
            if x > xmax {
                xmax = x;
            }

            if y < ymin {
                ymin = y;
            }
            if y > ymax {
                ymax = y;
            }
        }

        let y_diff = (ymax - ymin) * scale_factor;
        if scaled_row_y_max_height < y_diff {
            scaled_row_y_max_height = y_diff;
        }

        for vertex in mesh.iter_vertices() {
            let (x, y, z) = vertex.val();
            vertices.push([
                x * scale_factor + scaled_offset.x - xmin * scale_factor,
                y * scale_factor + scaled_offset.y,
                z * scale_factor,
            ]);
        }

        /*
        println!(
            " - x({:.3} - {:.3})={:.3}, y({:.3} - {:.3})={:.3}",
            xmin * scalar,
            xmax * scalar,
            (xmax - xmin) * scalar,
            ymin * scalar,
            ymax * scalar,
            (ymax - ymin) * scalar
        );
        */
        // 13 microsecs

        for normal in mesh.iter_normals().unwrap() {
            let (x, y, z) = normal.val();
            normals.push([x, y, z]);
        }
        // total = 24ms

        for face in mesh.iter_faces() {
            let val = face.val();
            indices.extend_from_slice(&[
                (val.0) as u32 + vertices_offset as u32,
                (val.1) as u32 + vertices_offset as u32,
                (val.2) as u32 + vertices_offset as u32,
            ]);
        }
        // 30 microsecs

        vertices_offset += mesh.vertices_len();

        scaled_offset.x += (xmax - xmin) * scale_factor + spacing.x;

        if text_mesh.size.wrapping && scaled_offset.x + scale_factor + spacing.x > line_end {
            scaled_offset.x = line_start;
            scaled_offset.y -= scaled_row_y_max_height + spacing.y;
        }

        //println!("");
    }

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
        let mut font = ttf2mesh::TTFFile::from_buffer_vec(get_font_bytes()).unwrap();

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
