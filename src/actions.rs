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