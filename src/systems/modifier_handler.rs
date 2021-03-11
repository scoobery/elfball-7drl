use crate::prelude::*;

pub fn apply_party_modifiers(party: &mut Vec<PartyMember>) {
    for member in party.iter_mut() {
        if !member.modifiers.is_empty() {
            for modifier in member.modifiers.iter_mut() {
                if !modifier.has_been_applied() {
                    match modifier.effect {
                        ModifierEffect::PlusAttack(a) => member.attack.set_modifier(a),
                        ModifierEffect::PlusThreat(t) => member.threat.set_modifier(t as u32),
                        ModifierEffect::PlusHealth(h) => {}
                    }
                    modifier.set_applied();
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
                        ModifierEffect::PlusHealth(_) => {}
                    }
                }
                else {
                    modifier.tick_down();
                }
            }
        }
    }
}