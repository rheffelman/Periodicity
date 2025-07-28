use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PropertiesEnum {
    Rect,
    Text,
    Healthbar,
    Castbar,
    State,
    TooltipData,
    Clickable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PId { // each entity must have 1 and only 1 PId
    pub id: u32,
    pub tag: String // will be specified for you if not specified.
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ColorPair {
    pub fill: (u8, u8, u8),
    pub outline: (u8, u8, u8),
}
impl ColorPair {
    pub fn from_colors(fill: sfml::graphics::Color, outline: sfml::graphics::Color) -> Self {
        ColorPair {
            fill: (fill.r, fill.g, fill.b),
            outline: (outline.r, outline.g, outline.b),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PRect {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub colors: ColorPair,
    pub pressed_color: Option<ColorPair>,
    pub hovered_color: Option<ColorPair>,
    pub pressed: Option<bool>,
    pub hovered: Option<bool>,
    pub draw: bool,
    pub strata: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PText {
    pub id: u32,
    pub text: String,
    pub scale: u32,
    pub x: u32,
    pub y: u32,
    pub colors: ColorPair,
    pub draw: bool,
    pub strata: u8
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PHealthbar {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub base_colors: ColorPair,
    pub inner_colors: ColorPair,
    pub draw: bool,
    pub strata: u8
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PCastbar {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub cast_progress: f32,
    pub base_colors: ColorPair,
    pub inner_colors: ColorPair,
    pub icon_name: String,
    pub draw: bool,
    pub strata: u8
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PState {
    pub id: u32,
    pub state_vec: Vec<u32>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PTooltipData {
    pub id: u32,
    pub header: String,
    pub body: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub icon: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PClickable {
    pub id: u32,
    pub clickable: bool,
    pub rect_reference_id: Option<u32>,
    pub action: ClickAction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ClickAction {
    RunButton,
    OtherButton,
    A,
    B,
    C,
    D,
}
