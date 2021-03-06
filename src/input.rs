use crate::prelude::*;

pub enum Actions {
    MoveUp,MoveDown,MoveLeft,MoveRight,
    MoveUpLeft,MoveUpRight,MoveDownLeft,MoveDownRight,
    TryGoDown,
    Wait,
}

//Grabs the player's keypresses
pub fn player_input(gs: &mut State, con: &BTerm) {
    match gs.status {
        ContextState::InGame => ingame_input(gs, con),
        _ => {}
    }
}

fn ingame_input(gs: &mut State, con: &BTerm) {

}