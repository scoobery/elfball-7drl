use crate::prelude::*;

pub enum Actions {
    MoveUp,MoveDown,MoveLeft,MoveRight,
    MoveUpLeft,MoveUpRight,MoveDownLeft,MoveDownRight,
    Wait, CycleTarget,
    ShowHelp
}

//Grabs the player's keypresses
pub fn player_input(gs: &mut State, con: &BTerm) {
    match gs.status {
        ContextState::InGame => ingame_input(gs, con),
        _ => {}
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

        Actions::ShowHelp => {
            gs.logs.update_logs(LogMessage::new()
                .add_part("=====================================================================================", ColorPair::new(WHITE,GREY10))
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