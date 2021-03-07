use crate::prelude::*;

pub fn spawn_player(pos: Point) -> Object {
    Object {
        name: String::from("Band of Heroic Elves"),
        floor: 1,
        tag: ActorTag::Player,
        pos: Some(pos),
        render: Some(Render::new(64, ColorPair::new(GOLD1, BLACK), 255)),
        viewshed: Some(Viewshed { range: 6, visible: Vec::new(), refresh: true }),
        ..Default::default()
    }
}


//Party Member definitions
pub fn make_hero() -> PartyMember {
    PartyMember {
        name: String::from("Elvish Hero"),
        class: String::from("Ranger"),
        icon: Render::new(2, ColorPair::new(GOLD,BLACK), 255),
        abilities: vec![Ability::Attack, Ability::RallyingCry],
        health: Health::new(20),
        attack: Attack::new(1,6),
        threat: Threat::new(6),
        modifiers: vec![Modifier::new(ModifierEffect::PlusAttack(1), 0, true)]
    }
}
pub fn make_guardian() -> PartyMember {
    PartyMember {
        name: String::from("Elvish Guardian"),
        class: String::from("Warrior"),
        icon: Render::new(2, ColorPair::new(STEEL_BLUE,BLACK), 255),
        abilities: vec![Ability::Attack, Ability::Taunt],
        health: Health::new(30),
        attack: Attack::new(2,3),
        threat: Threat::new(12),
        modifiers: Vec::new()
    }
}


//Enemy member definitions
pub fn enemy_make_forsaken_warrior() -> PartyMember {
    PartyMember {
        name: String::from("Forsaken Elf"),
        class: String::from("Warrior"),
        icon: Render::new(1, ColorPair::new(PURPLE,BLACK), 255),
        abilities: vec![Ability::Attack],
        health: Health::new(12),
        attack: Attack::new(2,3),
        threat: Threat::new(4),
        modifiers: Vec::new()
    }
}