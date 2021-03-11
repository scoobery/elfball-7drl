use crate::prelude::*;

pub fn check_player_collisions(gs: &mut State) {
    let pos = grab_position(&gs.world.objects);
    let idx = gs.world.map.point2d_to_index(pos);

    //Check if the player stepped on the portal
    if gs.world.map.tiles[idx] == TileClass::ForestPortal {
        gs.go_next_level = true;
        return
    }

    //Check if the player is trying to pick up some elves
    let mut maybe_add: Vec<PartyMember> = Vec::new();
    let mut remove_list: Vec<usize> = Vec::new();
    for (i, obj) in gs.world.objects.iter().enumerate() {
        if let Object { pos: Some(obj_pos), members: members, tag: ActorTag::Elf, .. } = obj {
            if pos == *obj_pos {
                for member in members {
                    maybe_add.push(member.clone());
                    remove_list.push(i);
                }
            }
        }
    }

    if !maybe_add.is_empty() && !remove_list.is_empty() {
        for e in maybe_add.iter() {
            let player = &mut gs.world.objects[0];

            if player.members.len() >= 10 {
                gs.logs.update_logs(LogMessage::new()
                    .add_part("Your party is full!", ColorPair::new(WHITE, GREY10))
                );
                return
            }
            else {
                add_member_to_party(e.clone(), player);
                gs.logs.update_logs(LogMessage::new()
                    .add_part(format!("{}", e.name), ColorPair::new(e.icon.get_render().1.fg, GREY10))
                    .add_part("has joined the party!", ColorPair::new(WHITE, GREY10))
                );
            }
        }
        for r in remove_list.iter() {
            gs.world.objects.remove(*r);
        }

        gs.player_targets.reset_targets(&gs.world.objects, &gs.world.map);
    }
}

fn grab_position(objects: &Vec<Object>) -> Point { return objects[0].pos.as_ref().unwrap().clone() }