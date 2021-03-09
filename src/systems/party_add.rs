use crate::prelude::*;

pub fn add_member_to_party(elf: PartyMember, player: &mut Object) {
    player.members.push(elf.clone());
}