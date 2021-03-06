mod state;
mod map;
mod camera;
mod render;
mod object;
mod input;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::state::*;
    pub use crate::map::*;
    pub use crate::camera::*;
    pub use crate::render::*;
    pub use crate::object::*;
    pub use crate::input::*;

    pub const CONSOLE_W: i32 = 80;
    pub const CONSOLE_H: i32 = 60;

    pub const BACK_CON: usize = 0;
    pub const MAP_CON: usize = 1;
    pub const TEXT_CON: usize = 2;
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