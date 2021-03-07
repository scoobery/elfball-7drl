use crate::prelude::*;

pub fn process_fov(objects: &mut Vec<Object>, map: &mut Map) {
    let mut fovlist: Vec<usize> = Vec::new();
    for (i, obj) in objects.iter().enumerate() {
        if obj.viewshed.is_some() {
            fovlist.push(i);
        }
    }

    for id in fovlist.iter() {
        let pos = objects[*id].pos.as_ref().unwrap().clone();
        let tag = objects[*id].tag.clone();
        let view = objects[*id].viewshed.as_mut().unwrap();

        if view.refresh {
            view.refresh = false;
            view.visible.clear();

            view.visible = field_of_view(pos, view.range, map);
            view.visible.retain(|p| {
                p.x >= 0 && p.x <= map.width - 1 && p.y >= 0 && p.y <= map.height - 1
            });

            if tag == ActorTag::Player {
                for t in map.visible.iter_mut() {
                    *t = false;
                }
                for v in view.visible.iter() {
                    let idx = map.point2d_to_index(*v);
                    map.revealed[idx] = true;
                    map.visible[idx] = true;
                }
            }
        }
    }
}