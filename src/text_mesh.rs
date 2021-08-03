use bevy::prelude::*;
use glyph_brush_layout::{HorizontalAlign, VerticalAlign};

use crate::{mesh_system::TextMeshState, TextMeshFont};
pub use ttf2mesh::Quality;

#[derive(Default, Bundle, Debug)]
pub struct TextMeshBundle {
    /// Text mesh configuration
    pub text_mesh: TextMesh,

    /// Standard bevy [`Transform`] for positioning the mesh
    pub transform: Transform,

    /// Standard bevy [`GlobalTransform`]
    pub global_transform: GlobalTransform,

    /// Internal mesh state, no public API
    pub text_mesh_state: TextMeshState,
}

/// Text mesh configuration
#[derive(Debug)]
pub struct TextMesh {
    /// Text string to be displayed
    pub text: String,

    /// Text styling options (incl. font size)
    pub style: TextMeshStyle,

    /// Text alignment options (**not implemented**)
    pub alignment: TextMeshAlignment,

    /// Text mesh container sizing
    pub size: TextMeshSize,
}

impl Default for TextMesh {
    fn default() -> Self {
        Self {
            text: "Hello World".into(),
            size: TextMeshSize::default(),
            style: TextMeshStyle::default(),
            alignment: Default::default(),
        }
    }
}

impl TextMesh {
    pub fn new<T: ToString>(text: T, font: Handle<TextMeshFont>) -> Self {
        Self {
            text: text.to_string(),
            style: TextMeshStyle {
                font,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_no_font<T: ToString>(text: T) -> Self {
        Self {
            text: text.to_string(),
            ..Default::default()
        }
    }

    pub fn new_with_color<T: ToString>(text: T, font: Handle<TextMeshFont>, color: Color) -> Self {
        Self {
            text: text.to_string(),
            style: TextMeshStyle {
                color,
                font,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

const DEFAULT_FONT_SIZE: f32 = 18.;

#[derive(Clone, Debug)]
pub struct TextMeshSize {
    pub width: SizeUnit,
    pub height: SizeUnit,
    pub depth: Option<SizeUnit>,
    pub wrapping: bool,
    pub overflow: bool,
}

impl Default for TextMeshSize {
    fn default() -> Self {
        Self {
            width: SizeUnit::NonStandard(DEFAULT_FONT_SIZE * 16.),
            height: SizeUnit::NonStandard(DEFAULT_FONT_SIZE * 8.),
            depth: Some(SizeUnit::NonStandard(DEFAULT_FONT_SIZE * 2.)),
            wrapping: true,
            overflow: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextMeshStyle {
    pub font: Handle<TextMeshFont>,
    pub font_size: SizeUnit,
    pub font_style: FontStyle,
    pub color: Color,
    pub mesh_quality: Quality,
}

impl Default for TextMeshStyle {
    fn default() -> Self {
        Self {
            font: Default::default(),
            font_size: SizeUnit::NonStandard(DEFAULT_FONT_SIZE),
            font_style: FontStyle::default(),
            color: Color::WHITE,
            mesh_quality: Quality::Medium,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SizeUnit {
    Auto,
    NonStandard(f32),
    //Px(f32),
    //Em(f32),
    //Cm(f32),
}

impl SizeUnit {
    pub fn as_scalar(&self) -> Option<f32> {
        match self {
            SizeUnit::Auto => None,
            SizeUnit::NonStandard(size) => Some(size / 144.),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextMeshAlignment {
    pub vertical: VerticalAlign,
    pub horizontal: HorizontalAlign,
}

impl Default for TextMeshAlignment {
    fn default() -> Self {
        TextMeshAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        }
    }
}

bitflags! {
    pub struct FontStyle: u32 {
        const BOLD = 0b1; // TODO: implement - another font?
        const ITALIC = 0b10; // TODO: implement - another font?
        const UNDERLINE = 0b100; // TODO: implement
        const STRIKETHROUGH = 0b1000; // TODO: implement
        const LOWERCASE = 0b10000;
        const UPPERCASE = 0b100000;
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::empty()
    }
}
