use crate::prelude::*;

pub fn spawn_player(pos: Point) -> Object {
    Object {
        name: String::from("Player"),
        floor: 1,
        pos: Some(pos),
        render: Some(Render::new(64, ColorPair::new(GOLD1, BLACK), 255)),
        viewshed: Some(Viewshed { range: 6, visible: Vec::new(), refresh: true }),
        ..Default::default()
    }
}