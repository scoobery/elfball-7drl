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
        for member in self.members.iter() {
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
        }
    }
}


//Abilities
#[derive(Clone, Copy, PartialEq)]
pub enum Ability {
    Taunt, CureWounds, RallyingCry, KillShot, Deforest, Block
}
pub struct StoredAbility {
    pub ability: Ability,
    pub name: String,
    pub min_cooldown: i32,
    pub cd_timer: i32,
    pub source_obj: usize,
    pub source_member: usize
}
impl StoredAbility {
    pub fn new(ability: Ability, source_obj: usize, source_member: usize) -> StoredAbility {
        let name = get_ability_name(ability);
        let min_cooldown = get_ability_cooldown(ability);

        StoredAbility {
            ability, name, min_cooldown, cd_timer: 0, source_obj, source_member
        }
    }
    pub fn increment_cd_timer(&mut self) {
        if self.cd_timer < self.min_cooldown { self.cd_timer += 1 }
        else { self.cd_timer = 0 }
    }
}

fn get_ability_name(ability: Ability) -> String {
    match ability {
        Ability::Taunt => String::from("Taunt"),
        Ability::Block => String::from("Block"),
        Ability::CureWounds => String::from("Cure Wounds"),
        Ability::RallyingCry => String::from("Rallying Cry"),
        Ability::KillShot => String::from("Kill Shot"),
        Ability::Deforest => String::from("Deforest"),
    }
}

fn get_ability_cooldown(ability: Ability) -> i32 {
    match ability {
        Ability::Taunt => 10,
        Ability::Block => 12,
        Ability::CureWounds => 5,
        Ability::RallyingCry => 18,
        Ability::KillShot => 20,
        Ability::Deforest => 20,
    }
}


pub fn handle_abilities(objects: &mut Vec<Object>, map: &mut Map, ability: &StoredAbility) {
    match ability.ability {
        Ability::Taunt => run_taunt(&mut objects[ability.source_obj].members[ability.source_member]),
        Ability::Block => run_block(&mut objects[ability.source_obj].members[ability.source_member]),
        Ability::CureWounds => {}
        Ability::RallyingCry => { }
        Ability::KillShot => {}
        Ability::Deforest => run_deforest(objects[ability.source_obj].pos.as_ref().unwrap(), map),
    }
    if let Some(view) = &mut objects[ability.source_obj].viewshed { view.refresh = true; }
}

fn run_taunt(member: &mut PartyMember) {
    member.modifiers.push(Modifier::new(ModifierEffect::PlusThreat(10), 6, false))
}
fn run_block(member: &mut PartyMember) {
    member.modifiers.push(Modifier::new(ModifierEffect::Block(5), 3, false))
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