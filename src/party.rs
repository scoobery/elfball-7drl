use crate::prelude::*;

pub struct PartyMember {
    pub name: String,
    pub class: String,
    pub abilities: Vec<Ability>,
    pub health: Health,
    pub attack: Attack,
    pub threat: Threat
}

pub enum Ability {
    Attack, Taunt, CureWounds
}

pub struct Health {
    max: u32,
    current: u32
}
impl Health {
    pub fn get_max(&self) -> u32 { return self.max }
    pub fn get_life(&self) -> u32 { return self.current }
    pub fn gain_life(&mut self, amt: u32) { self.current += amt }
    pub fn lose_life(&mut self, amt: u32) { self.current -= amt }
}

pub struct Attack {
    damage: (i32,i32)
}
impl Attack {
    pub fn roll_for_damage(&self, rng: &mut RandomNumberGenerator) -> i32 { return rng.roll_dice(self.damage.0, self.damage.1) }
}

pub struct Threat {
    current: u8,
    gain: u8
}
impl Threat {
    pub fn get_threat(&self) -> u8 { return self.current }
    pub fn reset_threat(&mut self) { self.current = 0 }
    pub fn increment_threat(&mut self) { self.current += self.gain }
}