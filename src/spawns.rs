use crate::prelude::*;

pub fn spawn_player(pos: Point) -> Object {
    Object {
        name: String::from("Band of Heroic Elves"),
        floor: 1,
        tag: ActorTag::Player,
        pos: Some(pos),
        render: Some(Render::new(64, ColorPair::new(GOLD1, BLACK), 255)),
        viewshed: Some(Viewshed { range: 6, visible: Vec::new(), refresh: true }),
        members: vec![make_guardian(), make_bard()],
        ..Default::default()
    }
}

pub fn spawn_band_of_forsaken(rng: &mut RandomNumberGenerator, pos: Point, f: u32) -> Object {
    let num_enemies = rng.range(f, (2 * f) + 1);
    Object {
        name: String::from("band of Forsaken Warriors"),
        floor: f,
        tag: ActorTag::Enemy,
        pos: Some(pos),
        render: Some(Render::new(1, ColorPair::new(PURPLE,BLACK), 255)),
        viewshed: Some(Viewshed { range: 6, visible: Vec::new(), refresh: true }),
        members: vec![enemy_make_forsaken_warrior(); num_enemies as usize],
        ai: Some(AIClass::new()),
        ..Default::default()
    }
}

pub fn spawn_elf_pickup(rng: &mut RandomNumberGenerator, pos: Point, f: u32) -> Object {
    let diceroll = rng.roll_dice(1, 2);
    let member = match diceroll {
        1 => vec![make_bard()],
        2 => vec![make_guardian()],
        _ => Vec::new()
    };

    Object {
        name: String::from("Lost Elf"),
        floor: f,
        block_tile: false,
        tag: ActorTag::Elf,
        pos: Some(pos),
        render: Some(Render::new(2, ColorPair::new(WHITE, BLACK), 254)),
        viewshed: None,
        members: member,
        ..Default::default()
    }
}

//Party Member definitions
pub fn make_bard() -> PartyMember {
    PartyMember {
        name: format!("{}", make_random_elf_name()),
        class: String::from("Bard"),
        icon: Render::new(2, ColorPair::new(GOLD,BLACK), 255),
        abilities: vec![AbilityClass::new(Ability::RallyingCry), AbilityClass::new(Ability::LesserCureWounds)],
        health: Health::new(20),
        attack: Attack::new(2,3),
        threat: Threat::new(4, 2),
        modifiers: vec![Modifier::new(ModifierEffect::PlusAttack(1), 0, true)],
    }
}
pub fn make_guardian() -> PartyMember {
    PartyMember {
        name: format!("{}", make_random_elf_name()),
        class: String::from("Guardian"),
        icon: Render::new(2, ColorPair::new(STEEL_BLUE,BLACK), 255),
        abilities: vec![AbilityClass::new(Ability::Taunt), AbilityClass::new(Ability::Block)],
        health: Health::new(30),
        attack: Attack::new(1,6),
        threat: Threat::new(6, 1),
        modifiers: Vec::new(),
    }
}


//Enemy member definitions
pub fn enemy_make_forsaken_warrior() -> PartyMember {
    PartyMember {
        name: String::from("Forsaken Elf"),
        class: String::from("Warrior"),
        icon: Render::new(1, ColorPair::new(PURPLE,BLACK), 255),
        abilities: vec![],
        health: Health::new(12),
        attack: Attack::new(1,4),
        threat: Threat::new(4, 2),
        modifiers: Vec::new(),
    }
}