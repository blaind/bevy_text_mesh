use bevy::prelude::*;
use ttf2mesh::{TTFFile, Value, Glyph};

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

// struct ttf_glyph
// {
//     /* general fields */
// 
//     int index;                    /* glyph index in font */
//     int symbol;                   /* utf-16 symbol */
//     int npoints;                  /* total points within all contours */
//     int ncontours;                /* number of contours in outline */
//     uint32_t composite : 1;       /* it is composite glyph */
//     uint32_t : 31;                /* reserved flags */
// 
//     /* horizontal glyph metrics */
//     /* see https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx */
// 
//     float xbounds[2];             /* min/max values ​​along the x coordinate */
//     float ybounds[2];             /* min/max values ​​along the y coordinate */
//     float advance;                /* advance width */
//     float lbearing;               /* left side bearing */
//     float rbearing;               /* right side bearing = aw - (lsb + xMax - xMin) */
// 
//     /* glyph outline */
// 
//     ttf_outline_t *outline;       /* original outline of the glyph or NULL */
// };

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
            internal_cache.as_mut().unwrap()
        }
    };

    // TODO performance: pre-allocate capacity
    let mut vertices = Vec::new(); //with_capacity(4308); // TODO: allocate opportunistically
    let mut normals = Vec::new(); //with_capacity(4308); // TODO: allocate opportunistically
    let mut indices = Vec::new(); //with_capacity(8520);

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

    let spacing = Vec2::new(0.08, 0.1) * scalar;

    let mut scaled_offset = Vec2::ZERO;
    let mut scaled_row_y_max_height = 0.;

    let tab_size = 4;
    let mut column = 0;

    for char in text.chars() {
        // println!("{} char [{}] column {}", i, char, column);

        // always get some glyph for metrics
        let mut glyph : Glyph = font.glyph_from_char('a').unwrap();

        let is_space = char == ' ';
        let is_tab = char == '\t';
        if is_space || is_tab {
            let times =
            if is_tab {
                let t = tab_size - (column % tab_size);
                // println!("{} tab times: {}", column, t);
                t
            } else { 1 };

            scaled_offset.x += (glyph.inner.advance * scalar) * times as f32;
            column += times;

            continue;
        } else {
            column += 1;
        }

        let key = CacheKey::new_3d(char, depth);

        let mesh = match cache.meshes.get(&key) {
            Some(mesh) => mesh,
            None => {
                let glyph_res = font.glyph_from_char(char);

                glyph = match glyph_res {
                    Ok(g) => g,
                    Err(_) => {
                        println!("Glyph {} not found", char);
                        font.glyph_from_char('?').unwrap()
                    }
                };

                let mesh = match &text_mesh.size.depth {
                    Some(unit) => glyph
                        .to_3d_mesh(text_mesh.style.mesh_quality, unit.as_scalar().unwrap())
                        .unwrap(),
                    None => todo!("2d glyphs are not implemented yet. Define depth"),
                };

                cache.meshes.insert(key.clone(), mesh);
                cache.meshes.get(&key).unwrap()
            }
        };

        // ttf glyph knows a ton of usefyl metrics
        // this one sets how much we move after symbol is rendered
        let advance = glyph.inner.advance * scalar;

        let ymin = glyph.inner.ybounds[0];
        let ymax = glyph.inner.ybounds[1];
        let y_diff = (ymax - ymin) * scalar;
        if scaled_row_y_max_height < y_diff {
            scaled_row_y_max_height = y_diff;
        }

        for vertex in mesh.iter_vertices() {
            let (x, y, z) = vertex.val();
            vertices.push([
                x * scalar + scaled_offset.x + glyph.inner.lbearing * scalar,
                y * scalar + scaled_offset.y,
                z * scalar,
            ]);
        }

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

        scaled_offset.x += advance;

        if text_mesh.size.wrapping
            && scaled_offset.x + scalar + spacing.x > text_mesh.size.width.as_scalar().unwrap()
        {
            scaled_offset.x = 0.;
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
