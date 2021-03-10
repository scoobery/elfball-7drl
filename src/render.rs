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
            batch_ui_draws(&gs.world.objects, &gs.logs);
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
                } else if map.revealed[idx] && object.player_mem.seen {
                    render_list.push((object, false))
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
        else {
            pos = obj.0.player_mem.last_pos.unwrap();
            color = ColorPair::new(GREY30, BLACK);
            batch.set(pos - offset, color, glyph);
        }
    }

    batch.submit(5000).expect("Failed to batch entity draw");
}

fn batch_ui_draws(objects: &Vec<Object>, logs: &LogBuffer) {
    let mut bg_batch = DrawBatch::new();
    let mut txt_batch = DrawBatch::new();
    bg_batch.target(MAP_CON);
    txt_batch.target(TEXT_CON);

    //UI Box definitions
    let ui_box: Rect = Rect::with_size(CONSOLE_W + 1 - UI_CUTOFF.x, 0, UI_CUTOFF.x - 2, CONSOLE_H - 1);
    let log_box: Rect = Rect::with_size(0, CONSOLE_H - UI_CUTOFF.y, CONSOLE_W - UI_CUTOFF.x, UI_CUTOFF.y - 1);

    //Draw both UI boxes
    bg_batch.draw_double_box(ui_box, ColorPair::new(GREY50, BLACK));
    bg_batch.draw_double_box(log_box, ColorPair::new(GREY50, BLACK));

    //Print headers for each section of the UI box
    txt_batch.fill_region(Rect::with_size(ui_box.x1 * 2 + 2, 1, (ui_box.width() * 2) - 3, 0),
                          ColorPair::new(LIME_GREEN,BLACK), 219);
    txt_batch.print_color(Point::new(ui_box.x1 * 2 + 4, 1),
                          "Party", ColorPair::new(BLACK, LIME_GREEN));

    txt_batch.fill_region(Rect::with_size(ui_box.x1 * 2 + 2, 21, (ui_box.width() * 2) - 3, 0),
                          ColorPair::new(RED,BLACK), 219);
    txt_batch.print_color(Point::new(ui_box.x1 * 2 + 4, 21),
                          "Combat", ColorPair::new(BLACK, RED));

    txt_batch.fill_region(Rect::with_size(ui_box.x1 * 2 + 2, 41, (ui_box.width() * 2) - 3, 0),
                          ColorPair::new(GOLD,BLACK), 219);
    txt_batch.print_color(Point::new(ui_box.x1 * 2 + 4, 41),
                          "Abilities", ColorPair::new(BLACK, GOLD));

    //Create the sub-boxes for party members, abilities, etc.
    let party_box = Rect::with_size(ui_box.x1 * 2 + 2, 2, (ui_box.width() * 2) - 3, 18);
    let combat_box = Rect::with_size(ui_box.x1 * 2 + 2, 22, (ui_box.width() * 2) - 3, 18);
    let ability_box = Rect::with_size(ui_box.x1 * 2 + 2, 42, (ui_box.width() * 2) - 3, 16);

    /*
    //Region fills for debugging purposes
    txt_batch.fill_region(party_box, ColorPair::new(DARK_GREEN, BLACK), 219);
    txt_batch.fill_region(combat_box, ColorPair::new(DARK_RED, BLACK), 219);
    txt_batch.fill_region(ability_box, ColorPair::new(DARK_GOLDENROD, BLACK), 219);
    */

    let party_sub_boxes = get_divider_boxes(&party_box);
    let combat_sub_boxes = get_divider_boxes(&combat_box);

    {
        let player_party = &objects[0].members;

        for (i, member) in player_party.iter().enumerate() {
            let sbox = party_sub_boxes[i];
            txt_batch.print_color(Point::new(sbox.x1, sbox.y1), format!("{}", member.name), ColorPair::new(member.icon.get_render().1.fg, BLACK));
            txt_batch.print_color(Point::new(sbox.x1, sbox.y1 + 1), format!("{}", to_char(member.icon.get_render().0 as u8)), ColorPair::new(member.icon.get_render().1.fg, BLACK));
        }
    }
    for sbox in combat_sub_boxes.iter() {
        /*
        txt_batch.fill_region(*sbox, ColorPair::new(WHITE, BLACK), 7);
        txt_batch.print_color(Point::new(sbox.x1, sbox.y1), format!("Member {}", i), ColorPair::new(BLACK, WHITE));
         */
    }

    //Print out the log buffer
    let mut tb = TextBlock::new(LOG_BOX.x1 * 2, LOG_BOX.y1, LOG_BOX.width() * 2, LOG_BOX.height());
    tb.print(&logs.format());
    tb.render_to_draw_batch(&mut txt_batch);

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

fn get_divider_boxes(source_rect: &Rect) -> Vec<Rect> {
    let mut boxes = Vec::new();
    let width_increment = source_rect.width()/4;
    let widths = vec![0, width_increment, width_increment * 2, width_increment * 3];

    for w in widths.iter() {
        boxes.push(Rect::with_size(source_rect.x1 + w, source_rect.y1, source_rect.width() / 4, 6));
        boxes.push(Rect::with_size(source_rect.x1 + w, source_rect.y1 + 6, source_rect.width() / 4, 6));
        boxes.push(Rect::with_size(source_rect.x1 + w, source_rect.y1 + 12, source_rect.width() / 4, 5));
    }

    return boxes
}