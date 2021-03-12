use crate::prelude::*;

pub enum Actions {
    MoveUp,MoveDown,MoveLeft,MoveRight,
    MoveUpLeft,MoveUpRight,MoveDownLeft,MoveDownRight,
    Wait, CycleTarget,
    ShowHelp,

    UseAbility1, UseAbility2, UseAbility3, UseAbility4, UseAbility5, UseAbility6, UseAbility7, UseAbility8, UseAbility9, UseAbility0,
    UseAbilityS1, UseAbilityS2, UseAbilityS3, UseAbilityS4, UseAbilityS5, UseAbilityS6, UseAbilityS7, UseAbilityS8, UseAbilityS9, UseAbilityS0,
}

//Grabs the player's keypresses
pub fn player_input(gs: &mut State, con: &BTerm) {
    match gs.status {
        ContextState::InGame => ingame_input(gs, con),
        ContextState::GameOver => game_over_input(gs, con),
    }
}

fn game_over_input(gs: &mut State, con: &BTerm) {
    if let Some(key) = con.key {
        match key {
            VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter => {
                *gs = State::init()
            },
            _ => {}
        }
    }
}

fn ingame_input(gs: &mut State, con: &BTerm) {
    if let Some(key) = con.key {
        match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H
            => process_action(gs, Actions::MoveLeft),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L
            => process_action(gs, Actions::MoveRight),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::J
            => process_action(gs, Actions::MoveUp),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::K
            => process_action(gs, Actions::MoveDown),

            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y
            => process_action(gs, Actions::MoveUpLeft),
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U
            => process_action(gs, Actions::MoveUpRight),
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B
            => process_action(gs, Actions::MoveDownLeft),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N
            => process_action(gs, Actions::MoveDownRight),

            VirtualKeyCode::Numpad5 | VirtualKeyCode::Period
            => process_action(gs, Actions::Wait),

            VirtualKeyCode::T
            => process_action(gs, Actions::CycleTarget),

            VirtualKeyCode::Slash
            => process_action(gs, Actions::ShowHelp),

            VirtualKeyCode::Key1
            => {
                if con.shift {
                    if !con.control { process_action(gs, Actions::UseAbilityS1) }
                    else { describe_ability(gs, 10) }
                }
                else {
                    if !con.control { process_action(gs, Actions::UseAbility1) }
                    else { describe_ability(gs, 0) }
                }
            },
            VirtualKeyCode::Key2
            => {
                if con.shift {
                    if !con.control { process_action(gs, Actions::UseAbilityS2) }
                    else { describe_ability(gs, 11) }
                }
                else {
                    if !con.control { process_action(gs, Actions::UseAbility2) }
                    else { describe_ability(gs, 1) }
                }
            },
            VirtualKeyCode::Key3
            => {
                if con.shift {
                    if !con.control { process_action(gs, Actions::UseAbilityS3) }
                    else { describe_ability(gs, 12) }
                }
                else {
                    if !con.control { process_action(gs, Actions::UseAbility3) }
                    else { describe_ability(gs, 2) }
                }
            },
            VirtualKeyCode::Key4
            => {
                if con.shift {
                    if !con.control { process_action(gs, Actions::UseAbilityS4) }
                    else { describe_ability(gs, 13) }
                }
                else {
                    if !con.control { process_action(gs, Actions::UseAbility4) }
                    else { describe_ability(gs, 3) }
                }
            },
            VirtualKeyCode::Key5
            => {
                if con.shift {
                    if !con.control { process_action(gs, Actions::UseAbilityS5) }
                    else { describe_ability(gs, 14) }
                }
                else {
                    if !con.control { process_action(gs, Actions::UseAbility5) }
                    else { describe_ability(gs, 4) }
                }
            },
            VirtualKeyCode::Key6
            => {
                if con.shift {
                    if !con.control { process_action(gs, Actions::UseAbilityS6) }
                    else { describe_ability(gs, 15) }
                }
                else {
                    if !con.control { process_action(gs, Actions::UseAbility6) }
                    else { describe_ability(gs, 5) }
                }
            },
            VirtualKeyCode::Key7
            => {
                if con.shift {
                    if !con.control { process_action(gs, Actions::UseAbilityS7) }
                    else { describe_ability(gs, 16) }
                }
                else {
                    if !con.control { process_action(gs, Actions::UseAbility7) }
                    else { describe_ability(gs, 6) }
                }
            },
            VirtualKeyCode::Key8
            => {
                if con.shift {
                    if !con.control { process_action(gs, Actions::UseAbilityS8) }
                    else { describe_ability(gs, 17) }
                }
                else {
                    if !con.control { process_action(gs, Actions::UseAbility8) }
                    else { describe_ability(gs, 7) }
                }
            },
            VirtualKeyCode::Key9
            => {
                if con.shift {
                    if !con.control { process_action(gs, Actions::UseAbilityS9) }
                    else { describe_ability(gs, 18) }
                }
                else {
                    if !con.control { process_action(gs, Actions::UseAbility9) }
                    else { describe_ability(gs, 8) }
                }
            },
            VirtualKeyCode::Key0
            => {
                if con.shift {
                    if !con.control { process_action(gs, Actions::UseAbilityS0) }
                    else { describe_ability(gs, 19) }
                }
                else {
                    if !con.control { process_action(gs, Actions::UseAbility0) }
                    else { describe_ability(gs, 9) }
                }
            },

            _ => {}
        }
    }
}

fn process_action(gs: &mut State, action: Actions) {
    let result = match action {
        Actions::MoveLeft => try_move_player(gs, DL_LEFT),
        Actions::MoveRight => try_move_player(gs, DL_RIGHT),
        Actions::MoveUp => try_move_player(gs, DL_UP),
        Actions::MoveDown => try_move_player(gs, DL_DOWN),

        Actions::MoveUpLeft => try_move_player(gs, DL_UP + DL_LEFT),
        Actions::MoveUpRight => try_move_player(gs, DL_UP + DL_RIGHT),
        Actions::MoveDownLeft => try_move_player(gs, DL_DOWN + DL_LEFT),
        Actions::MoveDownRight => try_move_player(gs, DL_DOWN + DL_RIGHT),

        Actions::Wait => true,

        Actions::CycleTarget => {
            gs.player_targets.reset_targets(&gs.world.objects, &gs.world.map);
            gs.player_targets.cycle_current_target();
            false
        },

        Actions::UseAbility1 => {
            if 0 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[0], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbility2 => {
            if 1 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[1], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbility3 => {
            if 2 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[2], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbility4 => {
            if 3 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[3], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbility5 => {
            if 4 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[4], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbility6 => {
            if 5 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[5], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbility7 => {
            if 6 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[6], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbility8 => {
            if 7 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[7], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbility9 => {
            if 8 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[8], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbility0 => {
            if 9 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[9], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbilityS1 => {
            if 10 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[10], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbilityS2 => {
            if 11 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[11], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbilityS3 => {
            if 12 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[12], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbilityS4 => {
            if 13 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[13], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbilityS5 => {
            if 14 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[14], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbilityS6 => {
            if 15 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[15], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbilityS7 => {
            if 16 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[16], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbilityS8 => {
            if 17 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[17], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbilityS9 => {
            if 18 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[18], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },
        Actions::UseAbilityS0 => {
            if 19 < gs.stored_abilities.len() {
                handle_abilities(&mut gs.world.objects, &mut gs.world.map, &mut gs.stored_abilities[19], &mut gs.world.rng, &mut gs.logs, gs.player_targets.get_current_target());
            }
            false
        },


        Actions::ShowHelp => {
            gs.logs.update_logs(LogMessage::new()
                .add_part("=====================================================================================", ColorPair::new(WHITE,GREY10))
            );
            gs.logs.update_logs(LogMessage::new()
                .add_part("You can also press", ColorPair::new(WHITE,GREY10))
                .add_part("Control", ColorPair::new(YELLOW,GREY10))
                .add_part("in conjunction with an ability key to see a description of what that ability does.", ColorPair::new(WHITE,GREY10))
            );
            gs.logs.update_logs(LogMessage::new()
                .add_part("Press the corresponding", ColorPair::new(WHITE,GREY10))
                .add_part("Numeric key", ColorPair::new(YELLOW,GREY10))
                .add_part("to use abilities listed in the sidebar (plus Shift if there is an S next to the number).", ColorPair::new(WHITE,GREY10))
            );
            gs.logs.update_logs(LogMessage::new()
                .add_part("Press", ColorPair::new(WHITE,GREY10))
                .add_part("T", ColorPair::new(RED,GREY10))
                .add_part("to cycle through visible targets.", ColorPair::new(WHITE,GREY10))
            );
            gs.logs.update_logs(LogMessage::new()
                .add_part("Press", ColorPair::new(WHITE,GREY10))
                .add_part("Numpad 5,", ColorPair::new(GREEN,GREY10))
                .add_part("or .", ColorPair::new(CYAN,GREY10))
                .add_part("to wait a turn.", ColorPair::new(WHITE,GREY10))
            );
            gs.logs.update_logs(LogMessage::new()
                .add_part("Use the", ColorPair::new(WHITE,GREY10))
                .add_part("Numpad (7,9,1,3),", ColorPair::new(GREEN,GREY10))
                .add_part("or Y,U,B,N", ColorPair::new(CYAN,GREY10))
                .add_part("to move diagonally.", ColorPair::new(WHITE,GREY10))
            );
            gs.logs.update_logs(LogMessage::new()
                .add_part("Use the", ColorPair::new(WHITE,GREY10))
                .add_part("arrow keys,", ColorPair::new(GOLD,GREY10))
                .add_part("Numpad (8,2,4,6),", ColorPair::new(GREEN,GREY10))
                .add_part("or Vim keys (J,K,H,L)", ColorPair::new(CYAN,GREY10))
                .add_part("to move ↑, ↓, ←, and →.", ColorPair::new(WHITE,GREY10))
            );
            gs.logs.update_logs(LogMessage::new()
                .add_part("Controls:", ColorPair::new(GOLD,GREY10))
            );
            gs.logs.update_logs(LogMessage::new()
                .add_part("=====================================================================================", ColorPair::new(WHITE,GREY10))
            );

            false
        },

        _ => false
    };
    gs.set_proc();
    gs.set_refresh();
    gs.passed_turn = result;
}

fn try_move_player(gs: &mut State, delta: Point) -> bool {
    let map = &gs.world.map;
    let camera = &mut gs.world.camera;
    let player = &mut gs.world.objects[0];

    let mut dest = player.pos.unwrap() + delta;

    player.try_move(dest, map);
    camera.move_camera(player.pos.unwrap());

    return if player.pos.unwrap() == dest { true } else { try_attack_player(gs, &mut dest) }
}

//Attempts to attack something
fn try_attack_player(gs: &mut State, dest: &mut Point) -> bool {
    let (player, all) = gs.world.objects.split_at_mut(1);
    let mut target: Option<&mut Object> = None;
    let mut tgt_id: Option<usize> = None;

    for (i, obj) in all.iter_mut().enumerate() {
        if let Object { pos: Some(pos), tag: tag, .. } = obj {
            if pos == dest && obj.floor == player[0].floor && tag == &mut ActorTag::Enemy {
                target = Some(obj);
                tgt_id = Some(i + 1);
            }
        }
    }

    return if let (Some(tgt), Some(id)) = (target, tgt_id) {
        player[0].try_attack(tgt, id, &mut gs.world.rng, &mut gs.logs);
        true
    } else {
        false
    }
}

fn describe_ability(gs: &mut State, ability_idx: usize) {
    if ability_idx < gs.stored_abilities.len() {
        let ability = &gs.stored_abilities[ability_idx];

        gs.logs.update_logs(LogMessage::new()
            .add_part("-------------------------------------------------------------------------------------", ColorPair::new(WHITE, GREY10))
        );
        gs.logs.update_logs(LogMessage::new()
            .add_part(format! {"{}", get_ability_description(ability.ability)}, ColorPair::new(WHITE, GREY10))
        );
        gs.logs.update_logs(LogMessage::new()
            .add_part(format! {"{}:", ability.name}, ColorPair::new(GOLD, GREY10))
        );
        gs.logs.update_logs(LogMessage::new()
            .add_part("-------------------------------------------------------------------------------------", ColorPair::new(WHITE, GREY10))
        );

        gs.set_proc();
        gs.set_refresh();
    }
}