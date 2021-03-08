use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct TargetedAttack {
    pub target: (usize, usize),
    pub damage: i32
}
impl TargetedAttack {
    pub fn new(target: (usize, usize), damage: i32) -> TargetedAttack { TargetedAttack { target, damage } }
}

pub fn process_combat(objects: &mut Vec<Object>, logs: &mut LogBuffer) {
    let mut attack_list: Vec<(usize, TargetedAttack)> = Vec::new();
    let mut kill_list: Vec<(usize, usize)> = Vec::new();

    //Save the targeted attack and the ID of the object triggering it
    for (i,obj) in objects.iter().enumerate() {
        if !obj.inc_attacks.is_empty() {
            for a in obj.inc_attacks.iter() {
                attack_list.push((i, a.clone()));
            }
        }
    }
    //Clear the incoming attacks
    for a in attack_list.iter() {
        objects[a.0].inc_attacks.clear();
    }
    //Process the damage against the targeted party member's health
    for a in attack_list.iter() {
        let target = &mut objects[a.1.target.0].members[a.1.target.1];
        target.health.lose_life(a.1.damage);

        if target.health.get_life() <= 0 {
            kill_list.push((a.1.target.0, a.1.target.1));
        }
    }
    //Kill anything that was added to the kill list
    for k in kill_list.iter() {
        let object_party = &mut objects[k.0].members;

        logs.update_logs(LogMessage::new()
            .add_part(format!("{}", object_party[k.1].name), ColorPair::new(object_party[k.1].icon.get_render().1.fg, GREY10))
            .add_part("has been slain.", ColorPair::new(WHITE, GREY10))
        );

        object_party.remove(k.1);

        //Remove the whole object if the party is empty (but also not the player)
        if object_party.is_empty() {
            if objects[k.0].tag != ActorTag::Player {
                objects.remove(k.0);
            }
            else {
                //Put game over logic here
            }
        }
    }
}