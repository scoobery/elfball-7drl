mod state;
mod map;
mod camera;
mod render;
mod object;
mod input;
mod spawns;
mod party;
mod actions;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::state::*;
    pub use crate::map::*;
    pub use crate::camera::*;
    pub use crate::render::*;
    pub use crate::object::*;
    pub use crate::input::*;
    pub use crate::spawns::*;
    pub use crate::party::*;
    pub use crate::actions::*;
    pub use crate::systems::*;
    use std::cmp::Reverse;

    pub const CONSOLE_W: i32 = 80;
    pub const CONSOLE_H: i32 = 60;

    pub const BACK_CON: usize = 0;
    pub const MAP_CON: usize = 1;
    pub const TEXT_CON: usize = 2;

    //Constant point values used as directional indicators
    pub const DL_LEFT: Point = Point { x: -1, y: 0 };
    pub const DL_RIGHT: Point = Point { x: 1, y: 0 };
    pub const DL_UP: Point = Point { x: 0, y: -1 };
    pub const DL_DOWN: Point = Point { x: 0, y: 1 };

    //Type alias for a vec of array indices and initiative values
    pub type InitList = Vec<(usize, u8)>;

    pub trait InitListTrait {
        fn add_object(&mut self, id: usize, init: u8);
        fn sort(&mut self);
    }
    impl InitListTrait for InitList {
        //Adds a new object to the list
        fn add_object(&mut self, id: usize, init: u8) {
            self.push((id, init));
        }
        //Sorts by descending initiative order
        fn sort(&mut self) {
            self.sort_by_key(|a| Reverse(a.1));
        }
    }

    pub trait Neighbor {
        fn get_neighbors(&self) -> Vec<Point>;
    }
    impl Neighbor for Point {
        fn get_neighbors(&self) -> Vec<Point> {
            return vec![
                *self + DL_UP,
                *self + DL_DOWN,
                *self + DL_LEFT,
                *self + DL_RIGHT,
                *self + DL_UP + DL_LEFT,
                *self + DL_UP + DL_RIGHT,
                *self + DL_DOWN + DL_LEFT,
                *self + DL_DOWN + DL_RIGHT
            ]
        }
    }
}
use crate::prelude::*;

embedded_resource!(TEXTFONT, "../res/text.png");
embedded_resource!(MAPFONT, "../res/map.png");
fn main() {
    link_resource!(TEXTFONT, "res/text.png");
    link_resource!(MAPFONT, "res/map.png");

    match main_loop(build_console(800, 640), State::init()) {
        Ok(_) => {}
        Err(e) => panic!("Could not initialize due to a fatal error:\n{}", e),
    }
}

fn build_console(w: i32, h: i32) -> BTerm {
    return BTermBuilder::new()
        .with_resource_path("res/")
        .with_font("map.png", 16, 16)
        .with_font("text.png", 8, 16)
        .with_title("Elfball")
        .with_tile_dimensions(16, 16)
        .with_dimensions(w / 16, h / 16)

        //Basic background console
        .with_simple_console(CONSOLE_W, CONSOLE_H, "map.png")
        //Console for map rendering
        .with_sparse_console(CONSOLE_W, CONSOLE_H, "map.png")
        //Text Console
        .with_sparse_console(CONSOLE_W * 2, CONSOLE_H, "text.png")

        .with_vsync(true)
        .build()
        .unwrap();
}