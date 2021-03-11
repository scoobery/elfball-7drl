use crate::prelude::*;

pub fn update_targets_in_vision(gs: &mut State) {
    if let Some(tgt_idx) = gs.player_targets.get_current_target().clone() {
        let visible = gs.world.objects[0].viewshed.as_ref().unwrap().visible.to_vec();
        let target_pos = gs.world.objects[tgt_idx].pos.as_ref().unwrap().clone();

        if !visible.contains(&target_pos) { gs.player_targets.reset_targets(&gs.world.objects, &gs.world.map); }
    }
    if gs.player_targets.possible_targets.is_empty() {
        gs.player_targets.reset_targets(&gs.world.objects, &gs.world.map);
    }
}

pub struct TargetList {
    current_target_index: Option<usize>,
    possible_targets: Vec<(usize, f32)>
}
impl TargetList {
    pub fn new() -> TargetList {
        TargetList {
            current_target_index: None,
            possible_targets: Vec::new()
        }
    }
    pub fn get_current_target(&self) -> Option<usize> {
        return if let Some(idx) = self.current_target_index {
            Some(self.possible_targets[idx].0)
        } else {
            None
        }
    }
    pub fn reset_targets(&mut self, objects: &Vec<Object>, map: &Map) {
        self.set_possible_targets(check_for_targets(objects, map));
        if self.possible_targets.len() > 0 {
            if self.current_target_index.is_none() {
                self.set_new_current_target(self.possible_targets.len() - 1)
            }
            else if let Some(idx) = self.current_target_index {
                if idx > self.possible_targets.len() - 1 { self.set_new_current_target(0) }
            }
        } else {
            self.current_target_index = None
        }
    }
    pub fn set_new_current_target(&mut self, new_idx: usize) { self.current_target_index = Some(new_idx) }
    pub fn cycle_current_target(&mut self) {
        if let Some(idx) = self.current_target_index {
            if idx == self.possible_targets.len() - 1 {
                self.current_target_index = Some(0)
            }
            else {
                *self.current_target_index.as_mut().unwrap() += 1
            }
        }
    }
    pub fn set_possible_targets(&mut self, target_list: Vec<(usize, f32)>) { self.possible_targets = target_list }
    pub fn num_targets(&self) -> usize { self.possible_targets.len() }
}

fn check_for_targets(objects: &Vec<Object>, map: &Map) -> Vec<(usize, f32)> {
    let player_pos = objects[0].pos.as_ref().unwrap().clone();
    let (enemy_positions, friend_positions) = grab_all_positions(objects);

    let mut enemy_distance_list = find_distance_of_positions(enemy_positions, map, player_pos);
    enemy_distance_list.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
    let mut friend_distance_list = find_distance_of_positions(friend_positions, map, player_pos);
    friend_distance_list.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

    let target_list = {
        let mut vec = Vec::new();
        vec.append(&mut enemy_distance_list);
        vec.append(&mut friend_distance_list);
        vec
    };

    return target_list
}

fn grab_all_positions(objects: &Vec<Object>) -> (Vec<(usize, Point)>, Vec<(usize, Point)>) {
    let vis_area = objects[0].viewshed.as_ref().unwrap().visible.to_vec();
    let mut enemy_positions: Vec<(usize, Point)> = Vec::new();
    let mut friend_positions: Vec<(usize, Point)> = Vec::new();

    for (i, obj) in objects.iter().enumerate() {
        if let Object { pos: Some(pos), tag, .. } = obj {
            if vis_area.contains(pos) {
                if tag == &ActorTag::Enemy {
                    enemy_positions.push((i, *pos));
                }
                else if tag == &ActorTag::Elf {
                    friend_positions.push((i, *pos));
                }
            }
        }
    }

    return (enemy_positions, friend_positions)
}

fn find_distance_of_positions(pos_vec: Vec<(usize, Point)>, map: &Map, player_pos: Point) -> Vec<(usize, f32)> {
    let mut distance_list: Vec<(usize, f32)> = Vec::new();
    let id_list: Vec<usize> = pos_vec.iter().map(|a| a.0).collect();
    let pos_list: Vec<Point> = pos_vec.iter().map(|a| a.1).collect();

    return pos_list.iter().enumerate()
        .map(|(i, pos)| (id_list[i], DistanceAlg::Pythagoras.distance2d(player_pos, *pos)))
        .collect::<Vec<(usize, f32)>>()
}