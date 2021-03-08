use crate::prelude::*;

pub fn update_blocked_tiles(objects: &Vec<Object>, map: &mut Map, floor: u32) {
    for b in map.obj_blocked.iter_mut() {
        *b = false;
    }

    for obj in objects.iter() {
        if let Object{ pos: Some(pos), .. } = obj {
            let block = &obj.block_tile;

            if *block && obj.floor == floor {
                let idx = map.point2d_to_index(*pos);
                map.obj_blocked[idx] = true;
            }
        }
    }
}