use crate::prelude::*;

pub struct Object {
    pub floor: u32,
    pub pos: Option<Point>,
    pub render: Option<Render>
}

//Component Definitions
pub struct Render {
    glyph: FontCharType,
    color: ColorPair,
    pub order: u8
}
impl Render {
    pub fn new(glyph: FontCharType, color: ColorPair, order: u8) -> Render { Render { glyph, color, order } }
    pub fn get_render(&self) -> (FontCharType, ColorPair) { return (self.glyph, self.color) }
}