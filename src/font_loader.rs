use anyhow::Result;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::reflect::{TypePath, TypeUuid};
use bevy::text::Font;

#[derive(Default)]
pub struct FontLoader;

impl AssetLoader for FontLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            // standard bevy_text/src/font_loader code
            let font = Font::try_from_bytes(bytes.into())?;
            load_context.set_default_asset(LoadedAsset::new(font));

            // ttf fontloading
            let font = TextMeshFont {
                ttf_font: ttf2mesh::TTFFile::from_buffer_vec(bytes.to_vec()).unwrap(),
            };

            load_context.set_labeled_asset("mesh", LoadedAsset::new(font));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ttf"]
    }
}

#[derive(TypeUuid, TypePath)]
#[uuid = "5415ac03-d009-471e-89ab-dc0d4e31a8c4"]
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
