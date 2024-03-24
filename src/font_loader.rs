use bevy::asset::AsyncReadExt;
use bevy::text::Font;
use std::error::Error;
use std::fmt::Display;

use anyhow::Result;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, BoxedFuture, LoadContext};
use bevy::reflect::TypePath;

#[derive(Debug)]
pub struct FontLoaderError;

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
        _: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader
                .read_to_end(&mut bytes)
                .await
                .expect("unable to read font");

            // ttf fontloading
            let font = TextMeshFont {
                ttf_font: ttf2mesh::TTFFile::from_buffer_vec(bytes.clone())
                    .expect("unable to decode asset"),
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

#[derive(TypePath, Asset)]
// #[uuid = "5415ac03-d009-471e-89ab-dc0d4e31a8c4"]
pub struct TextMeshFont {
    pub(crate) ttf_font: ttf2mesh::TTFFile,
}

impl std::fmt::Debug for TextMeshFont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TextMeshFont<>")
    }
}

unsafe impl Sync for TextMeshFont {} // FIXME - verify the soundness
unsafe impl Send for TextMeshFont {} // FIXME - verify the soundness
