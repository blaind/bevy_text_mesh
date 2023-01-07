use bevy::asset::AsyncReadExt;
use bevy::text::Font;
use glyph_brush_layout::ab_glyph::InvalidFont;
use std::error::Error;
use std::fmt::Display;

use anyhow::Result;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, BoxedFuture, LoadContext};
use bevy::reflect::{TypePath, TypeUuid};

#[derive(Debug)]
pub struct FontLoaderError;

impl From<InvalidFont> for FontLoaderError {
    fn from(_value: InvalidFont) -> Self {
        Self
    }
}

impl Error for FontLoaderError {}

impl Display for FontLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

#[derive(Default)]
pub struct FontLoader;

impl AssetLoader for FontLoader {
    type Asset = Font;
    type Settings = ();
    type Error = FontLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _s: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader
                .read_to_end(&mut bytes)
                .await
                .expect("unable to read font");
            // standard bevy_text/src/font_loader code
            let font = Font::try_from_bytes(bytes.clone())?;
            load_context.add_labeled_asset("font".into(), font);

            let common =
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789?!.,".to_string();

            let mut generator = meshtext::MeshGenerator::new(bytes.clone());
            generator.precache_glyphs(&common, false, None).unwrap();

            // ttf fontloading
            let font = TextMeshFont {
                ttf_font_generator: generator,
            };

            load_context.add_labeled_asset("mesh".into(), font);

            let original_font = Font::try_from_bytes(bytes.into()).expect("unable to read font");

            Ok(original_font)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ttf"]
    }
}

#[derive(TypeUuid, TypePath, Asset)]
#[uuid = "5415ac03-d009-471e-89ab-dc0d4e31a8c4"]
pub struct TextMeshFont {
    pub(crate) ttf_font_generator: meshtext::MeshGenerator<meshtext::OwnedFace>,
}

impl std::fmt::Debug for TextMeshFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TextMeshFont<>")
    }
}

unsafe impl Sync for TextMeshFont {} // FIXME - verify the soundness
unsafe impl Send for TextMeshFont {} // FIXME - verify the soundness
