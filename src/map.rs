use crate::prelude::*;
use std::thread::spawn;

#[derive(Clone,Copy,PartialEq)]
pub enum TileClass {
    Tree, ForestFloor
}
impl TileClass {
    pub fn does_collide(&self) -> bool { return *self == TileClass::Tree }
    pub fn does_blos(&self) -> bool { return *self == TileClass::Tree }
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub starting_pos: Point,
    pub exit_pos: Point,
    pub tiles: Vec<TileClass>,
    pub visible: Vec<bool>,
    pub revealed: Vec<bool>,
    pub obj_blocked: Vec<bool>
}
impl Map {
    pub fn new(w: i32, h: i32) -> Map {
        Map {
            width: w,
            height: h,
            starting_pos: Point::zero(),
            exit_pos: Point::zero(),
            tiles: vec![TileClass::Tree; (w * h) as usize],
            visible: vec![true; (w * h) as usize],
            revealed: vec![false; (w * h) as usize],
            obj_blocked: vec![false; (w * h) as usize],
        }
    }

    //Checks if something is within the map's boundaries
    pub fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }
    //Checks to see if the tile at an index is walkable
    pub fn walkable(&self, pos: Point) -> bool {
        let idx = self.point2d_to_index(pos);
        return self.in_bounds(pos) && !self.tiles[idx].does_collide() && !self.obj_blocked[idx]
    }

    fn valid_exit(&self, pos: Point, delta: Point) -> Option<usize> {
        let dest = pos + delta;

        if self.in_bounds(dest) {
            if self.walkable(dest) {
                let idx = self.point2d_to_index(dest);
                return Some(idx)
            }
            else {
                return None
            }
        }
        else {
            return None
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool { return self.tiles[idx] != TileClass::ForestFloor }
}
impl Algorithm2D for Map {
    fn dimensions(&self) -> Point { Point::new(self.width, self.height) }
}


pub fn cellular_automata_builder(w: i32, h: i32, start_mid: bool) -> Map {
    let mut map = Map::new(w, h);
    let mut rng = RandomNumberGenerator::new();

    //Generate a random mishmash of walls and floors
    for y in 1..map.height-1 {
        for x in 1..map.width-1 {
            let pos = Point::new(x,y);
            let idx = map.point2d_to_index(pos);
            let d100 = rng.roll_dice(1, 100);
            if d100 > 25 { map.tiles[idx] = TileClass::ForestFloor; }
        }
    }

    //Run the cellular automata algorithm against it
    for _ in 0..23 {
        let mut new_tilemap = map.tiles.to_vec();
        for y in 1..map.height-1 {
            for x in 1..map.width-1 {
                let pos = Point::new(x,y);
                let idx = map.point2d_to_index(pos);

                let neighbors = get_neighbors_count(&map, idx);
                if neighbors > 4 || neighbors == 0 { new_tilemap[idx] = TileClass::Tree }
                else { new_tilemap[idx] = TileClass::ForestFloor }
            }
        }
        map.tiles = new_tilemap.to_vec();
    }

    //If the starting position should be the middle of the map
    if start_mid {
        let mut pos = Point::new(map.width/2, map.height/2);
        let mut idx = map.point2d_to_index(pos);

        while map.tiles[idx] != TileClass::ForestFloor {
            pos.x -= 1;
            idx = map.point2d_to_index(pos);
        }
        map.starting_pos = pos;
    }
    //Otherwise, start at the furthest point from the middle
    else {
        let start_idx = vec![map.point2d_to_index(Point::new(map.width/2, map.height/2))];
        let dijkstra_map = DijkstraMap::new(map.width, map.height, &start_idx, &map, 1024.0);
        let mut spawn_tile = (0, 0.0f32);
        for (i, tile) in map.tiles.iter_mut().enumerate() {
            let distance = dijkstra_map.map[i];
            if distance > spawn_tile.1 {
                spawn_tile.0 = i;
                spawn_tile.1 = distance;
            }
        }
        map.starting_pos = map.index_to_point2d(spawn_tile.0);
    }

    return map
}
fn get_neighbors_count(map: &Map, idx: usize) -> u8 {
    let mut final_val = 0;
    if map.tiles[idx - 1] == TileClass::Tree { final_val += 1 }
    if map.tiles[idx + 1] == TileClass::Tree { final_val += 1 }
    if map.tiles[idx - map.width as usize] == TileClass::Tree { final_val += 1 }
    if map.tiles[idx + map.width as usize] == TileClass::Tree { final_val += 1 }
    if map.tiles[idx - (map.width as usize - 1)] == TileClass::Tree { final_val += 1 }
    if map.tiles[idx + (map.width as usize - 1)] == TileClass::Tree { final_val += 1 }
    if map.tiles[idx - (map.width as usize + 1)] == TileClass::Tree { final_val += 1 }
    if map.tiles[idx + (map.width as usize + 1)] == TileClass::Tree { final_val += 1 }
    return final_val
}