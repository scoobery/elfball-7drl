use crate::prelude::*;

pub struct Object {
    pub name: String,
    pub floor: u32,
    pub block_tile: bool,
    pub tag: ActorTag,

    pub pos: Option<Point>,
    pub render: Option<Render>,
    pub viewshed: Option<Viewshed>,

    pub player_mem: PlayerMemory,
    pub inc_attacks: Vec<TargetedAttack>,

    pub members: Vec<PartyMember>
}
impl Default for Object {
    fn default() -> Self {
        Object {
            name: String::from("Nil"),
            floor: 0,
            block_tile: true,
            tag: ActorTag::NonActor,
            pos: None,
            render: None,
            viewshed: None,
            player_mem: PlayerMemory::default(),
            inc_attacks: Vec::new(),
            members: Vec::new(),
        }
    }
}

//Component Definitions
#[derive(Clone, Copy, PartialEq)]
pub enum ActorTag {
    NonActor, Player, Enemy, Static
}

#[derive(Clone)]
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

pub struct PlayerMemory {
    pub seen: bool,
    pub last_pos: Option<Point>
}
impl Default for PlayerMemory {
    fn default() -> Self { PlayerMemory { seen: false, last_pos: None } }
}