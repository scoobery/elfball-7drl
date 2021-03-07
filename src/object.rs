use crate::prelude::*;

pub struct Object {
    pub name: String,
    pub floor: u32,
    pub pos: Option<Point>,
    pub render: Option<Render>,
    pub viewshed: Option<Viewshed>,
    pub members: Option<Vec<PartyMember>>
}
impl Default for Object {
    fn default() -> Self {
        Object {
            name: String::from("Nil"),
            floor: 0,
            pos: None,
            render: None,
            viewshed: None,
            members: None
        }
    }
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

pub struct Viewshed {
    pub range: i32,
    pub visible: Vec<Point>,
    pub refresh: bool
}