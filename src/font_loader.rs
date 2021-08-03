use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::reflect::TypeUuid;

#[derive(Default)]
pub struct FontLoader;

impl AssetLoader for FontLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, anyhow::Result<()>> {
        Box::pin(async move {
            let font = TextMeshFont {
                ttf_font: ttf2mesh::TTFFile::from_buffer_vec(bytes.to_vec()).expect("TTFFile::from_buffer_vec"),
            };

            load_context.set_default_asset(LoadedAsset::new(font));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ttf"]
    }
}

#[derive(TypeUuid)]
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
