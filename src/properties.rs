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
    rect
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PPos {
    pub x: u32,
    pub y: u32
}

#[derive(Debug, Clone)]
pub struct PRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub c: Color,
    pub draw: bool
}