use crate::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ContextState {
    MainMenu, InGame, Paused, GameMenu
}
#[derive(Clone, Copy, PartialEq)]
pub enum TurnState {
    Player, AI, GameOver
}

pub struct State {
    proc: bool,
    refresh: bool,
    pub passed_turn: bool,
    pub status: ContextState,
    pub turn_state: TurnState,
    pub world: World
}
impl State {
    pub fn init() -> State {
        State {
            proc: true,
            refresh: true,
            passed_turn: false,
            status: ContextState::InGame,
            turn_state: TurnState::Player,
            world: World::new_game()
        }
    }
    pub fn set_proc(&mut self) { self.proc = true }
    pub fn set_refresh(&mut self) { self.refresh = true }
}
impl GameState for State {
    fn tick(&mut self, con: &mut BTerm) {
        if self.turn_state == TurnState::Player { player_input(self, con) }
        else if self.turn_state == TurnState::GameOver { /*Do the other thing once it's ready*/ }

        match self.status {
            ContextState::MainMenu => {}
            ContextState::Paused => {}
            ContextState::GameMenu => {}
            ContextState::InGame => {
                exec_all_systems(self);
            }
        }

        if self.refresh {
            render_loop(&self, con);
            self.refresh = false;
        }
    }
}


fn exec_all_systems(gs: &mut State) {
    if gs.proc {
        //Execute the systems and shit

        if gs.passed_turn {
            gs.turn_state == TurnState::AI;
            gs.passed_turn = false;
            //Don't forget to run FOV here
        }

        if gs.turn_state == TurnState::AI {
            //Do all the ai stuff
            gs.turn_state = TurnState::Player;
        }

        gs.proc = false;
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

        world.objects.push(spawn_player(starting_pos));

        return world
    }
}