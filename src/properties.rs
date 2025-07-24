use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, Text, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::{Event, Style};
use sfml::window::mouse::Button;

use std::fs;
use std::mem;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PropertiesEnum {
    pos,
    rect,
    text,
    sprite
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PPos {
    pub x: u32,
    pub y: u32
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub draw: bool,
    pub strata: u8
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PText {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub text: String,
    pub scale: u32,
    pub x: u32,
    pub y: u32,
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