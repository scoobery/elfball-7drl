use crate::prelude::*;

impl Object {
    pub fn try_move(&mut self, dest: Point, map: &Map) {
        if self.pos.is_some() {
            if !map.walkable(dest) {
                return
            }
            else {
                self.pos = Some(dest);
                if let Object { viewshed: Some(view), .. } = self {
                    view.refresh = true
                }
            }
        }
        else {
            console::log("ERROR: Entity attempted to move without positional component.")
        }
    }
}