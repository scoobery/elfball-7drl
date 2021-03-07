use crate::prelude::*;

pub struct PartyMember {
    pub name: String,
    pub class: String,
    pub icon: Render,
    pub abilities: Vec<Ability>,
    pub health: Health,
    pub attack: Attack,
    pub threat: Threat,
    pub modifiers: Vec<Modifier>
}

pub enum Ability {
    Attack, Taunt, CureWounds, RallyingCry
}

pub struct Modifier {
    pub effect: ModifierEffect,
    ttl: u32,
    permanent: bool
}
impl Modifier {
    pub fn new(effect: ModifierEffect, ttl: u32, permanent: bool) -> Modifier { Modifier { effect, ttl, permanent } }
    pub fn tick_down(&mut self) { if !self.permanent { self.ttl -= 1 } }
    pub fn ttl_is_zero(&self) -> bool {
        if self.permanent {
            false
        } else {
            self.ttl == 0
        }
    }
}
pub enum ModifierEffect {
    PlusAttack(i32), PlusThreat(i32), PlusHealth(i32)
}


pub struct Health {
    max: u32,
    current: u32
}
impl Health {
    pub fn new(max: u32) -> Health { Health { current: max, max } }
    pub fn get_max(&self) -> u32 { return self.max }
    pub fn get_life(&self) -> u32 { return self.current }
    pub fn gain_life(&mut self, amt: u32) { self.current += amt }
    pub fn lose_life(&mut self, amt: u32) { self.current -= amt }
}

pub struct Attack {
    damage: (i32,i32)
}
impl Attack {
    pub fn new(num: i32, d: i32) -> Attack { Attack { damage: (num,d) } }
    pub fn roll_for_damage(&self, rng: &mut RandomNumberGenerator) -> i32 { return rng.roll_dice(self.damage.0, self.damage.1) }
}

pub struct Threat {
    current: u32,
    gain: u32
}
impl Threat {
    pub fn new(threat: u32) -> Threat { Threat { current: 0, gain: threat } }
    pub fn get_threat(&self) -> u32 { return self.current }
    pub fn reset_threat(&mut self) { self.current = 0 }
    pub fn increment_threat(&mut self) { self.current += self.gain }
}