use crate::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ContextState {
    InGame, GameMenu
}
#[derive(Clone, Copy, PartialEq)]
pub enum TurnState {
    Player, AI, GameOver
}

pub struct State {
    proc: bool,
    refresh: bool,
    pub passed_turn: bool,
    pub go_next_level: bool,
    pub player_death: bool,
    pub status: ContextState,
    pub turn_state: TurnState,
    pub world: World,
    pub logs: LogBuffer,
    pub player_targets: TargetList,
    pub stored_abilities: Vec<StoredAbility>,
    pub highlighted_ability: Option<usize>
}
impl State {
    pub fn init() -> State {
       let mut logs = LogBuffer::new();
       logs.update_logs(LogMessage::new()
           .add_part("Press", ColorPair::new(WHITE, GREY10))
           .add_part("Slash (/)", ColorPair::new(GOLD, GREY10))
           .add_part("to view the controls at any time.", ColorPair::new(WHITE, GREY10))
       );
       logs.update_logs(LogMessage::new()
           .add_part("Your ancestors have called upon you to save these lands from the bestial scourge they are beset against.", ColorPair::new(WHITE, GREY10))
           .add_part("Gather more", ColorPair::new(WHITE, GREY10))
           .add_part("Elves (☻)", ColorPair::new(LIME_GREEN, GREY10))
           .add_part("to expand your party of woodland defenders.", ColorPair::new(WHITE, GREY10))
           .add_part("Beware of", ColorPair::new(WHITE, GREY10))
           .add_part("Beasts (b)", ColorPair::new(RED, GREY10))
           .add_part("that stalk these woods, and", ColorPair::new(WHITE, GREY10))
           .add_part("the Forsaken (☺),", ColorPair::new(PURPLE, GREY10))
           .add_part("your fallen bretheren. Look for", ColorPair::new(WHITE, GREY10))
           .add_part("Portals (§)", ColorPair::new(CYAN, GREY10))
           .add_part("to travel to new parts of the forest. Good luck...", ColorPair::new(WHITE, GREY10))
       );

       State {
           proc: true,
           refresh: true,
           passed_turn: false,
           go_next_level: false,
           player_death: false,
           status: ContextState::InGame,
           turn_state: TurnState::Player,
           world: World::new_game(),
           logs,
           player_targets: TargetList::new(),
           stored_abilities: Vec::new(),
           highlighted_ability: None
       }
    }

    pub fn refresh_stored_abilities(&mut self) {
        self.stored_abilities.clear();
        for (i, member) in self.world.objects[0].members.iter().enumerate() {
            for ability in member.abilities.iter() {
                self.stored_abilities.push(StoredAbility::new(*ability, 0, i));
            }
        }
        if self.stored_abilities.len() > 0 {
            if self.highlighted_ability.is_none() {
                self.highlighted_ability = Some(0);
            }
        }
        else {
            self.highlighted_ability = None;
        }
        self.set_refresh();
    }

    pub fn set_proc(&mut self) { self.proc = true }
    pub fn set_refresh(&mut self) { self.refresh = true }
}
impl GameState for State {
    fn tick(&mut self, con: &mut BTerm) {
        if self.turn_state == TurnState::Player { player_input(self, con) }
        else if self.turn_state == TurnState::GameOver { /*Do the other thing once it's ready*/ }

        match self.status {
            ContextState::GameMenu => {}
            ContextState::InGame => {
                exec_all_systems(self);
            }
        }

        if self.refresh {
            render_loop(&self, con);
            self.refresh = false;
        }

        if self.go_next_level {
            self.go_next_level = false;
            self.world.generate_new_map();
            self.player_targets.reset_targets(&self.world.objects, &self.world.map);
            self.set_proc();
            self.set_refresh();
        }

        self.refresh_stored_abilities();
    }
}


fn exec_all_systems(gs: &mut State) {
    if gs.proc {
        gs.proc = false;

        apply_party_modifiers(&mut gs.world.objects[0].members);

        //Execute the systems and shit
        process_fov(&mut gs.world.objects, &mut gs.world.map);
        process_combat(&mut gs.world.objects, &mut gs.logs, &mut gs.player_death, &mut gs.player_targets, &gs.world.map);
        update_blocked_tiles(&mut gs.world.objects, &mut gs.world.map, gs.world.depth);
        check_player_collisions(gs);

        if gs.passed_turn {
            gs.turn_state = TurnState::AI;
            gs.passed_turn = false;
            process_fov(&mut gs.world.objects, &mut gs.world.map);
        }

        if gs.turn_state == TurnState::AI {
            process_ai(&mut gs.world.objects, &mut gs.world.map, gs.world.depth, &mut gs.world.rng, &mut gs.logs);
            process_fov(&mut gs.world.objects, &mut gs.world.map);
            process_combat(&mut gs.world.objects, &mut gs.logs, &mut gs.player_death, &mut gs.player_targets, &gs.world.map);
            update_blocked_tiles(&mut gs.world.objects, &mut gs.world.map, gs.world.depth);
            gs.turn_state = TurnState::Player;
        }

        update_player_memory(&mut gs.world.objects);
        update_targets_in_vision(gs);

        clean_party_modifiers(&mut gs.world.objects[0].members);

        if gs.player_death {
            gs.turn_state = TurnState::GameOver;
            println!("Game is done!");
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
        let mut objects = Vec::new();
        let mut map = cellular_automata_builder(64,64, true);
        let camera = Camera::new(map.starting_pos.clone());
        objects.push(spawn_player(map.starting_pos.clone()));

        for _ in 1..=12 {
            let max_roll = map.valid_spawns.len() - 1;
            let index = rng.range(0, max_roll);
            let pos = map.valid_spawns[index].clone();
            objects.push(spawn_band_of_forsaken(&mut rng, pos, 1));
            map.valid_spawns.remove(index);
        }

        for _ in 1..=3 {
            let max_roll = map.valid_spawns.len() - 1;
            let index = rng.range(0, max_roll);
            let pos = map.valid_spawns[index].clone();
            objects.push(spawn_elf_pickup(&mut rng, pos, 1));
            map.valid_spawns.remove(index);
        }

        let mut world = World {
            rng,
            objects,
            map,
            depth: 1,
            camera
        };

        return world
    }

    pub fn generate_new_map(&mut self) {
        self.objects.retain(|o| o.tag == ActorTag::Player);
        self.depth += 1;
        let mut new_map = cellular_automata_builder(64,64, true);

        self.camera = Camera::new(new_map.starting_pos.clone());
        self.objects[0].pos = Some(new_map.starting_pos.clone());
        self.objects[0].viewshed.as_mut().unwrap().refresh = true;
        self.objects[0].floor = self.depth;

        for _ in 1..=10 + (2 * self.depth) {
            let max_roll = new_map.valid_spawns.len() - 1;
            let index = self.rng.range(0, max_roll);
            let pos = new_map.valid_spawns[index].clone();
            self.objects.push(spawn_band_of_forsaken(&mut self.rng, pos, self.depth));
            new_map.valid_spawns.remove(index);
        }

        for _ in 1..=3 {
            let max_roll = new_map.valid_spawns.len() - 1;
            let index = self.rng.range(0, max_roll);
            let pos = new_map.valid_spawns[index].clone();
            self.objects.push(spawn_elf_pickup(&mut self.rng, pos, self.depth));
            new_map.valid_spawns.remove(index);
        }

        self.map = new_map;
    }
}