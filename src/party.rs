use crate::prelude::*;

#[derive(Clone)]
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

#[derive(Clone, Copy, PartialEq)]
pub enum Ability {
    Attack, Taunt, CureWounds, RallyingCry
}

#[derive(Clone)]
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
#[derive(Clone, Copy, PartialEq)]
pub enum ModifierEffect {
    PlusAttack(i32), PlusThreat(i32), PlusHealth(i32)
}

#[derive(Clone)]
pub struct Health {
    max: i32,
    current: i32
}
impl Health {
    pub fn new(max: i32) -> Health { Health { current: max, max } }
    pub fn get_max(&self) -> i32 { return self.max }
    pub fn get_life(&self) -> i32 { return self.current }
    pub fn gain_life(&mut self, amt: i32) { self.current += amt }
    pub fn lose_life(&mut self, amt: i32) { self.current -= amt }
}

#[derive(Clone)]
pub struct Attack {
    damage: (i32,i32),
    able_to_attack: bool
}
impl Attack {
    pub fn new(num: i32, d: i32) -> Attack { Attack { damage: (num,d), able_to_attack: true } }
    pub fn roll_for_damage(&self, rng: &mut RandomNumberGenerator) -> i32 { return rng.roll_dice(self.damage.0, self.damage.1) }
    pub fn is_able(&self) -> bool { self.able_to_attack }
    pub fn disable_attack(&mut self) { self.able_to_attack = false }
    pub fn enable_attack(&mut self) { self.able_to_attack = true }
}

#[derive(Clone)]
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


pub fn make_random_elf_name() -> String {
    let mut rng = RandomNumberGenerator::new();

    let rand1 = rng.range(1, 101);
    let rand2 = rng.range(1, 101);

    let string1 =
        if rand1 >= 1 && rand1 <= 10 { "Kal" }
        else if rand1 >= 11 && rand1 <= 20 { "Shen" }
        else if rand1 >= 21 && rand1 <= 30 { "Faen" }
        else if rand1 >= 31 && rand1 <= 40 { "Tyr" }
        else if rand1 >= 41 && rand1 <= 50 { "Malek" }
        else if rand1 >= 51 && rand1 <= 60 { "Hyrin" }
        else if rand1 >= 61 && rand1 <= 70 { "Mar" }
        else if rand1 >= 71 && rand1 <= 80 { "Kor" }
        else if rand1 >= 81 && rand1 <= 90 { "Eldra" }
        else if rand1 >= 91 && rand1 <= 100 { "Aelin" }
        else {"!!!"};

    let string2 =
        if rand2 >= 1 && rand2 <= 10 { "'dael" }
        else if rand2 >= 11 && rand2 <= 20 { "'thas" }
        else if rand2 >= 21 && rand2 <= 30 { "'thar" }
        else if rand2 >= 31 && rand2 <= 40 { "'ano" }
        else if rand2 >= 41 && rand2 <= 50 { "'ymnath" }
        else if rand2 >= 51 && rand2 <= 60 { "'myn" }
        else if rand2 >= 61 && rand2 <= 70 { "'lyron" }
        else if rand2 >= 71 && rand2 <= 80 { "'inor" }
        else if rand2 >= 81 && rand2 <= 90 { "'thyl" }
        else if rand2 >= 91 && rand2 <= 100 { "'barad" }
        else {"!!!"};

    return String::from(format!("{}{}", string1, string2))
}