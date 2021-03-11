use crate::prelude::*;

pub fn apply_party_modifiers(party: &mut Vec<PartyMember>) {
    for member in party.iter_mut() {
        if !member.modifiers.is_empty() {
            for modifier in member.modifiers.iter_mut() {
                if !modifier.has_been_applied() {
                    match modifier.effect {
                        ModifierEffect::PlusAttack(a) => member.attack.set_modifier(a),
                        ModifierEffect::PlusThreat(t) => member.threat.set_modifier(t as u32),
                        ModifierEffect::Block(b) => member.health.set_block(b),
                    }
                    modifier.set_applied();
                    //println!("Applying modifier to {}!", member.name);
                }
            }
        }
    }
}

pub fn clean_party_modifiers(party: &mut Vec<PartyMember>) {
    for member in party.iter_mut() {
        if !member.modifiers.is_empty() {
            for modifier in member.modifiers.iter_mut() {
                if modifier.ttl_is_zero() {
                    match modifier.effect {
                        ModifierEffect::PlusAttack(_) => member.attack.reset_modifier(),
                        ModifierEffect::PlusThreat(_) => member.threat.reset_modifier(),
                        ModifierEffect::Block(_) => member.health.reset_block(),
                    }
                    //println!("Removing modifier from {}!", member.name);
                }
                else {
                    modifier.tick_down();
                }
            }
        }
    }
}