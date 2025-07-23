use sfml::cpp::FBox;
use sfml::graphics::{Color, Font, Text, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::{Event, Style};
use sfml::window::mouse::Button;

use std::fs;
use std::mem;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Components {
    CPos,
    CRect
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CPos {
    pub x: u32,
    pub y: u32
}
#[derive(Debug, Clone)]
pub struct CRect<'a> {
    pub rect: RectangleShape<'a>,
}