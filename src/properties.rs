use sfml::cpp::FBox;
use sfml::graphics::glsl::Vec3;
use sfml::graphics::{Color, Font, Text, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::{Event, Style};
use sfml::window::mouse::Button;

use std::fs;
use std::mem;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PropertiesEnum {
    rect,
    text,
    sprite,
    stat,
    healthbar,
    castbar,
    state
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ColorPair {
    pub fill: (u8, u8, u8),
    pub outline: (u8, u8, u8),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub colors: ColorPair,
    pub draw: bool,
    pub strata: u8
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PText {
    pub text: String,
    pub scale: u32,
    pub x: u32,
    pub y: u32,
    pub colors: ColorPair,
    pub draw: bool,
    pub strata: u8
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PSprite {
    pub x: u32,
    pub y: u32,
    pub scale: u32,
    pub sprite_name: String,
    pub draw: bool,
    pub strata: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PStats {
    pub health_max: u32,
    pub health_curr: u32,
    pub chaos: u32,
    pub solidity: u32,
    pub vitality: u32,
    pub haste: u32,
    pub will: u32,
    pub volatility: u32
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PHealthbar {
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
    pub state_vec: Vec<u32>
}