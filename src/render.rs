use crate::prelude::*;

pub fn render_loop(gs: &State, con: &mut BTerm) {
    con.cls();
    render_draw_buffer(con).expect("Failed to render");
    match gs.status {
        ContextState::MainMenu => {}
        ContextState::GameMenu => {}
        ContextState::Paused => {}
        ContextState::InGame => {
            batch_map_draws(&gs.world.map, &gs.world.camera);
            batch_entity_draws(&gs.world.objects, &gs.world.map, &gs.world.camera, gs.world.depth);
            batch_ui_draws();
        }
    }
    render_draw_buffer(con).expect("Failed to render");
}

//Adds all map tiles to the rendering batch.
fn batch_map_draws(map: &Map, camera: &Camera) {
    let mut batch = DrawBatch::new();
    batch.target(MAP_CON);
    let offset = Point::new(camera.min_x, camera.min_y);

    for y in camera.min_y ..= camera.max_y {
        for x in camera.min_x ..= camera.max_x {
            let pos = Point::new(x, y);

            if map.in_bounds(pos) {
                let idx = map.point2d_to_index(pos);

                let (glyph, colors) = match (map.visible[idx], map.revealed[idx]) {
                    (true, _) => {get_tile_render(&map.tiles[idx])},
                    (false, true) => {(get_tile_render(&map.tiles[idx]).0, ColorPair::new(GREY10,BLACK))},
                    (false, false) => {(0,ColorPair::new(BLACK,BLACK))},
                };

                batch.set(pos - offset, colors, glyph);
            }
            else {
                batch.set(pos - offset, ColorPair::new(BLACK, BLACK), 0);
            }
        }
    }
    batch.submit(0).expect("Failed to batch map draw");
}

//Adds all visible entity renderables to the rendering batch.
fn batch_entity_draws(objects: &Vec<Object>, map: &Map, camera: &Camera, floor: u32) {
    let mut batch = DrawBatch::new();
    batch.target(MAP_CON);
    let offset = Point::new(camera.min_x, camera.min_y);

    //Grab all objects that are drawable and have a position (force the player in at the end)
    let mut render_list: Vec<(&Object, bool)> = Vec::new();
    for object in objects.iter() {
        if object.pos.is_some() && object.render.is_some() {
            let pos = object.pos.as_ref().unwrap();
            let idx = map.point2d_to_index(*pos);
            if pos.x > camera.min_x && pos.x < camera.max_x && pos.y > camera.min_y && pos.y < camera.max_y && object.floor == floor {
                if map.visible[idx] {
                    render_list.push((object, true))
                //UNCOMMENT THIS LATER WHEN I IMPLEMENT PLAYER MEMORY AGAIN!!!
                //} else if map.revealed[idx] && object.player_mem.seen {
                //    render_list.push((object, false))
                }
            }
        }
    }

    render_list.sort_by_key(|o| o.0.render.as_ref().unwrap().order);
    for obj in render_list.iter() {
        let pos: Point;
        let (glyph, mut color) = obj.0.render.as_ref().unwrap().get_render();

        if obj.1 {
            pos = obj.0.pos.unwrap();
            batch.set(pos - offset, color, glyph);
        }
        /* ALSO UNCOMMENT THIS WHEN I IMPLEMENT PLAYER MEMORY AGAIN
        else {
            pos = obj.0.player_mem.last_pos.unwrap();
            color = ColorPair::new(GREY30, BLACK);
            batch.set(pos - offset, color, glyph);
        }
        */
    }

    batch.submit(5000).expect("Failed to batch entity draw");
}

fn batch_ui_draws() {
    let mut bg_batch = DrawBatch::new();
    let mut txt_batch = DrawBatch::new();
    bg_batch.target(MAP_CON);
    txt_batch.target(TEXT_CON);

    let ui_box: Rect = Rect::with_size(CONSOLE_W + 1 - UI_CUTOFF.x, 0, UI_CUTOFF.x - 2, CONSOLE_H - 1);
    let log_box: Rect = Rect::with_size(0, CONSOLE_H - UI_CUTOFF.y, CONSOLE_W - UI_CUTOFF.x, UI_CUTOFF.y - 1);
    bg_batch.draw_double_box(ui_box, ColorPair::new(GREY50, BLACK));
    bg_batch.draw_double_box(log_box, ColorPair::new(GREY50, BLACK));
    txt_batch.print(Point::new(ui_box.x1 * 2 + 2, 1), "Elfball");

    bg_batch.submit(10000).expect("Failed to batch UI draw");
    txt_batch.submit(11000).expect("Failed to batch UI draw");
}

fn get_tile_render(tile: &TileClass) -> (FontCharType, ColorPair) {
    match tile {
        TileClass::ForestFloor => (46, ColorPair::new(BROWN1,BLACK)),
        TileClass::Tree => (5, ColorPair::new(GREEN,BLACK)),
        TileClass::ForestPortal => (21, ColorPair::new(CYAN,BLACK)),
    }
}