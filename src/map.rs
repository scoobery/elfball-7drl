use crate::prelude::*;

#[derive(Clone,Copy,PartialEq)]
pub enum TileClass {
    Tree, ForestFloor, ForestPortal
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
    pub starting_area: Vec<Point>,
    pub tiles: Vec<TileClass>,
    pub visible: Vec<bool>,
    pub revealed: Vec<bool>,
    pub obj_blocked: Vec<bool>,
    pub valid_spawns: Vec<Point>
}
impl Map {
    pub fn new(w: i32, h: i32) -> Map {
        Map {
            width: w,
            height: h,
            starting_pos: Point::zero(),
            exit_pos: Point::zero(),
            starting_area: Vec::new(),
            tiles: vec![TileClass::Tree; (w * h) as usize],
            visible: vec![false; (w * h) as usize],
            revealed: vec![false; (w * h) as usize],
            obj_blocked: vec![false; (w * h) as usize],
            valid_spawns: Vec::new()
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

    pub fn get_valid_spawn_points(&mut self) {
        let mut points = Vec::new();

        for y in 1 ..= self.height - 1 {
            for x in 1 ..= self.width - 1 {
                points.push(Point::new(x,y));
            }
        }

        points.retain(|p| !self.starting_area.contains(p));
        points.retain(|p| self.walkable(*p));

        self.valid_spawns = points;
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool { return self.tiles[idx] != TileClass::ForestFloor }
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize,f32); 10]> {
        let mut exits = SmallVec::new();
        let pos = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(pos, DL_LEFT) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(pos, DL_RIGHT) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(pos, DL_UP) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(pos, DL_DOWN) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(pos, DL_UP + DL_LEFT) { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(pos, DL_DOWN + DL_LEFT) { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(pos, DL_UP + DL_RIGHT) { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(pos, DL_DOWN + DL_RIGHT) { exits.push((idx, 1.45)) }

        return exits
    }
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
        let start_pt = Point::new(map.width/2, map.height/2);
        map.starting_pos = find_furthest_point(&mut map, &start_pt);
    }

    //Set the starting area on the map
    map.starting_area = bresenham_filled_circle_graph(&map.starting_pos, 9).to_vec();
    map.get_valid_spawn_points();

    //Set up where the exit portal will be located
    let start_pt = map.starting_pos.clone();
    map.exit_pos = find_furthest_point(&mut map, &start_pt);
    let exit_idx = map.point2d_to_index(map.exit_pos);
    map.tiles[exit_idx] = TileClass::ForestPortal;

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

fn find_furthest_point(map: &mut Map, pos: &Point) -> Point {
    let start_idx = vec![map.point2d_to_index(*pos)];
    let dijkstra_map = DijkstraMap::new(map.width, map.height, &start_idx, map, 8192.0);

    let start_tile = dijkstra_map.map
        .iter()
        .enumerate()
        .filter(|(_,dist)| *dist < &f32::MAX)
        .max_by(|a,b| a.1.partial_cmp(b.1).unwrap())
        .unwrap().0;

    return map.index_to_point2d(start_tile)
}

fn bresenham_filled_circle_graph(pos: &Point, radius: i32) -> Vec<Point> {
    let mut points = Vec::new();
    for i in 0..=radius {
        BresenhamCircle::new(*pos, i).for_each(|p| { points.push(p); });
    }
    points.dedup();
    return points
}