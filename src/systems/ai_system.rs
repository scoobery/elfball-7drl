use crate::prelude::*;

pub fn process_ai(objects: &mut Vec<Object>, map: &mut Map, floor: u32, rng: &mut RandomNumberGenerator, logs: &mut LogBuffer) {
    let (player, all) = objects.split_at_mut(1);
    let player_pos = player[0].pos.unwrap();
    let mut proclist = Vec::new();

    for (id, obj) in all.iter().enumerate() {
        if let Object{ tag, .. } = obj {
            if *tag == ActorTag::Enemy {
                proclist.push(id);
            }
        }
    }

    for unit in proclist.iter() {
        basic_enemy_ai(*unit, objects, map, rng, logs, player_pos);
        update_blocked_tiles(objects, map, floor);
    }
}

fn basic_enemy_ai(enemy_id: usize, objects: &mut Vec<Object>, map: &Map, rng: &mut RandomNumberGenerator, logs: &mut LogBuffer, player_pos: Point) {
    let (player, all) = &mut objects.split_at_mut(1);
    let enemy = &mut all[enemy_id];
    let player = &mut player[0];
    let pos = enemy.pos.unwrap();

    if enemy.floor == player.floor {
        if let Object { viewshed: Some(view), ai: Some(ai), .. } = enemy {
            if view.visible.contains(&player_pos) && enemy.floor == player.floor {
                enemy.in_combat = true;
                player.in_combat = true;

                ai.target = Some(0);
                ai.state = AIState::Chasing;
                ai.tgt_memory = 24;
                ai.tgt_heatmap.reset_to_single_node(&player_pos);

                let mut dest: Point = pos;
                let distance = DistanceAlg::Pythagoras.distance2d(pos, player_pos);
                let targets = vec![map.point2d_to_index(player_pos)];
                let dijkstra_map = DijkstraMap::new(90, 90, &targets, map, 1024.0);

                if let Some(destidx) = DijkstraMap::find_lowest_exit(&dijkstra_map, map.point2d_to_index(pos), map) {
                    dest = if distance > 1.45 {
                        map.index_to_point2d(destidx)
                    } else {
                        player_pos
                    };
                }
                if distance <= 1.45 {
                    enemy.try_attack(player, 0, rng, logs);
                } else if dest != pos { enemy.try_move(dest, map) }
            } else if ai.tgt_memory > 0 {
                enemy.in_combat = false;
                ai.state = AIState::Hunting;
                ai.tgt_memory -= 1;
                ai.tgt_heatmap.spread(pos, map);
                let dest = ai.tgt_heatmap.get_closest_heat(map, pos);
                if dest != pos { enemy.try_move(dest, map) }
            } else {
                enemy.in_combat = false;
                ai.target = None;
                ai.state = AIState::Idle;
            }
        }
        clear_ai_heatmap(enemy);
    }
}

fn clear_ai_heatmap(enemy: &mut Object) {
    if let Object { viewshed: Some(view), ai: Some(ai), ..} = enemy {
        ai.tgt_heatmap.clear_heat_area(&view.visible);
    }
}