use crate::prelude::*;

pub enum ContextState {
    MainMenu, InGame, Paused, GameMenu
}

pub struct State {
    proc: bool,
    refresh: bool,
    pub status: ContextState,
    pub world: World
}
impl State {
    pub fn init() -> State {
        State {
            proc: true,
            refresh: true,
            status: ContextState::InGame,
            world: World::new_game()
        }
    }
    pub fn set_proc(&mut self) { self.proc = true }
}
impl GameState for State {
    fn tick(&mut self, con: &mut BTerm) {
        if self.refresh {
            render_loop(&self, con);
            render_draw_buffer(con).expect("Failed to render");
            self.refresh = false;
        }

        if self.proc {
            //Do all the things!
            self.proc = false;
        }
    }
}


pub struct World {
    pub rng: RandomNumberGenerator,
    pub objects: Vec<Object>,
    pub map: Map,
    pub depth: u32,
    pub camera: Camera
}
impl World {
    pub fn empty() -> World {
        World {
            rng: RandomNumberGenerator::new(),
            objects: Vec::new(),
            map: Map::new(0,0),
            depth: 0,
            camera: Camera::new(Point::zero()),
        }
    }
    pub fn new_game() -> World {
        let mut rng = RandomNumberGenerator::new();
        let map = cellular_automata_builder(120, 120, true);
        let starting_pos = map.starting_pos.clone();

        let mut world = World {
            rng,
            objects: Vec::new(),
            map,
            depth: 1,
            camera: Camera::new(starting_pos)
        };

        return world
    }
}