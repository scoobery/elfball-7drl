use crate::prelude::*;

impl Object {
    pub fn try_move(&mut self, dest: Point, map: &Map) {
        if self.pos.is_some() {
            if !map.walkable(dest) {
                return
            }
            else {
                self.pos = Some(dest);
                if let Object { viewshed: Some(view), .. } = self {
                    view.refresh = true
                }
            }
        }
        else {
            console::log("ERROR: Entity attempted to move without positional component.")
        }
    }

    pub fn try_attack(&mut self, target: &mut Object, target_id: usize, rng: &mut RandomNumberGenerator, logs: &mut LogBuffer) {
        for member in self.members.iter_mut() {
            if member.attack.is_able() {
                let mut attack_target = 0;
                let mut threat_num = 0;

                //TODO: Add more party member behaviours which will determine who their priority targets are?
                for (i, other_member) in target.members.iter().enumerate() {
                    if other_member.threat.get_threat() > threat_num {
                        threat_num = other_member.threat.get_threat();
                        attack_target = i;
                    }
                }

                let damage = member.attack.roll_for_damage(rng);
                self.inc_attacks.push(TargetedAttack::new((target_id, attack_target), damage));

                logs.update_logs(LogMessage::new()
                    .add_part(format!("{}", member.name), ColorPair::new(member.icon.get_render().1.fg, GREY10))
                    .add_part("attacks", ColorPair::new(WHITE, GREY10))
                    .add_part(format!("{}", target.members[attack_target].name), ColorPair::new(target.members[attack_target].icon.get_render().1.fg, GREY10))
                    .add_part(format!("for {} damage.", damage), ColorPair::new(WHITE, GREY10))
                );
            }
            else {
                member.attack.enable_attack();
            }
        }
    }
}


//Abilities
#[derive(Clone, Copy, PartialEq)]
pub enum Ability {
    Taunt, CureWounds, LesserCureWounds, RallyingCry, KillShot, Deforest, Block
}
pub struct StoredAbility {
    pub ability: Ability,
    pub name: String,
    pub on_cooldown: bool,
    pub source_obj: usize,
    pub source_member: usize,
    pub source_ability_id: usize
}
impl StoredAbility {
    pub fn new(ability: Ability, source_obj: usize, source_member: usize, source_ability_id: usize, on_cooldown: bool) -> StoredAbility {
        let name = get_ability_name(ability);
        StoredAbility {
            ability, name, on_cooldown, source_obj, source_member, source_ability_id
        }
    }
    pub fn is_on_cooldown(&self) -> bool { self.on_cooldown }
    pub fn set_source_on_cooldown(&mut self, objects: &mut Vec<Object>) {
        let ability_source = &mut objects[self.source_obj].members[self.source_member].abilities[self.source_ability_id];
        ability_source.set_on_cooldown();
    }
}

pub fn get_ability_name(ability: Ability) -> String {
    match ability {
        Ability::Taunt => String::from("Taunt"),
        Ability::Block => String::from("Block"),
        Ability::CureWounds => String::from("Cure Wounds"),
        Ability::LesserCureWounds => String::from("Lesser Cure"),
        Ability::RallyingCry => String::from("Rallying Cry"),
        Ability::KillShot => String::from("Kill Shot"),
        Ability::Deforest => String::from("Deforest"),
    }
}

pub fn get_ability_cooldown(ability: Ability) -> i32 {
    match ability {
        Ability::Taunt => 10,
        Ability::Block => 10,
        Ability::CureWounds => 15,
        Ability::LesserCureWounds => 15,
        Ability::RallyingCry => 15,
        Ability::KillShot => 20,
        Ability::Deforest => 16,
    }
}


pub fn handle_abilities(objects: &mut Vec<Object>, map: &mut Map, ability: &mut StoredAbility, rng: &mut RandomNumberGenerator, logs: &mut LogBuffer) {
    if !ability.is_on_cooldown() {
        match ability.ability {
            Ability::Taunt => run_taunt(&mut objects[ability.source_obj].members[ability.source_member], logs),
            Ability::Block => run_block(&mut objects[ability.source_obj].members[ability.source_member], logs),
            Ability::CureWounds => run_cure_wounds(&mut objects[ability.source_obj].members, ability.source_member, rng, logs, false),
            Ability::LesserCureWounds => run_cure_wounds(&mut objects[ability.source_obj].members, ability.source_member, rng, logs, true),
            Ability::RallyingCry => {}
            Ability::KillShot => {}
            Ability::Deforest => run_deforest(objects[ability.source_obj].pos.as_ref().unwrap(), map),
        }
        ability.set_source_on_cooldown(objects);
    }
    else {
        logs.update_logs(LogMessage::new()
            .add_part(format!("{}'s", objects[ability.source_obj].members[ability.source_member].name), ColorPair::new(objects[ability.source_obj].members[ability.source_member].icon.get_render().1.fg, GREY10))
            .add_part(format!("{} is still on cooldown!", ability.name), ColorPair::new(WHITE, GREY10))
        );
    }
    if let Some(view) = &mut objects[ability.source_obj].viewshed { view.refresh = true; }
}

fn run_taunt(member: &mut PartyMember, logs: &mut LogBuffer) {
    member.modifiers.push(Modifier::new(ModifierEffect::PlusThreat(10), 6, false));
    logs.update_logs(LogMessage::new()
        .add_part(format!("{}",member.name), ColorPair::new(member.icon.get_render().1.fg, GREY10))
        .add_part("lets out a threatening shout, taunting enemies to attack them!", ColorPair::new(WHITE, GREY10))
    );
}
fn run_block(member: &mut PartyMember, logs: &mut LogBuffer) {
    member.modifiers.push(Modifier::new(ModifierEffect::Block(5), 2, false));
    logs.update_logs(LogMessage::new()
        .add_part(format!("{}",member.name), ColorPair::new(member.icon.get_render().1.fg, GREY10))
        .add_part("raises their shield, blocking the enemies' blows!", ColorPair::new(WHITE, GREY10))
    );
    member.attack.disable_attack();
}
fn run_cure_wounds(members: &mut Vec<PartyMember>, caster_id: usize, rng: &mut RandomNumberGenerator, logs: &mut LogBuffer, lesser: bool) {
    let health_list = {
        let mut vec = members.iter().enumerate()
            .map(|(i, m)| (i, m.health.get_max() - m.health.get_life()))
            .collect::<Vec<(usize, i32)>>();
        vec.sort_by(|a,b| b.1.cmp(&a.1));
        vec
    };
    let amt = if !lesser { rng.roll_dice(3,4) }
        else{ rng.roll_dice(2, 3) };
    members[health_list[0].0].health.gain_life(amt);

    logs.update_logs(LogMessage::new()
        .add_part(format!("{}", members[caster_id].name.clone()), ColorPair::new(members[caster_id].icon.get_render().1.fg, GREY10))
        .add_part("casts a healing wave upon", ColorPair::new(WHITE, GREY10))
        .add_part(format!("{}", members[health_list[0].0].name), ColorPair::new(members[health_list[0].0].icon.get_render().1.fg, GREY10))
        .add_part("for", ColorPair::new(WHITE, GREY10))
        .add_part(format!("{}", amt), ColorPair::new(GOLD, GREY10))
        .add_part("HP.", ColorPair::new(WHITE, GREY10))
    );

    members[caster_id].attack.disable_attack();
}

fn run_deforest(source_pos: &Point, map: &mut Map) {
    let neighbor_list = {
        let mut vec = Vec::new();
        //let mut second_vec = Vec::new();

        vec.append(&mut source_pos.get_neighbors());
        /*
        for p in vec.iter() {
            second_vec.append(&mut p.get_neighbors());
        }
        vec.append(&mut second_vec);
        vec.dedup();
         */
        vec.retain(|p| p.x >= 1 && p.y >= 1 && p.x <= map.width - 2 && p.y <= map.height - 2);

        vec
    };

    let idx_list = neighbor_list.iter()
        .map(|i| map.point2d_to_index(*i))
        .collect::<Vec<usize>>();

    for idx in idx_list.into_iter() {
        if map.tiles[idx] == TileClass::Tree { map.tiles[idx] = TileClass::ForestFloor }
    }
}