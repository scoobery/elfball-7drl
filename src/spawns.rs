use crate::prelude::*;
use std::cmp::min;

pub fn spawn_player(pos: Point) -> Object {
    Object {
        name: String::from("Band of Heroic Elves"),
        floor: 1,
        tag: ActorTag::Player,
        pos: Some(pos),
        render: Some(Render::new(64, ColorPair::new(GOLD1, BLACK), 255)),
        viewshed: Some(Viewshed { range: 6, visible: Vec::new(), refresh: true }),
        members: vec![make_guardian(), make_hunter()],
        ..Default::default()
    }
}

pub fn spawn_band_of_forsaken(rng: &mut RandomNumberGenerator, pos: Point, f: u32) -> Object {
    let num_enemies = rng.range(f, min(f + 1, 5));
    let band_vec = {
        let mut vec = Vec::new();
        for _ in 0..=num_enemies {
            let roll = rng.roll_dice(1,10);
            match roll {
                1|2|3|4|5|6|7|8 => vec.push(enemy_make_forsaken_warrior(rng, f)),
                9|10 => vec.push(enemy_make_forsaken_caster(rng, f)),
                _ => panic!("WTF BOOM")
            }
        }
        vec
    };

    Object {
        name: String::from("band of Forsaken Warriors"),
        floor: f,
        tag: ActorTag::Enemy,
        pos: Some(pos),
        render: Some(Render::new(1, ColorPair::new(PURPLE,BLACK), 255)),
        viewshed: Some(Viewshed { range: 6, visible: Vec::new(), refresh: true }),
        members: band_vec,
        ai: Some(AIClass::new()),
        ..Default::default()
    }
}

pub fn spawn_beast(rng: &mut RandomNumberGenerator, pos: Point, f: u32) -> Object {
    let hp_mod = 10 * f as i32;
    Object {
        name: String::from("Forgotten Beast"),
        floor: f,
        tag: ActorTag::Enemy,
        pos: Some(pos),
        render: Some(Render::new(98, ColorPair::new(RED, BLACK), 255)),
        viewshed: Some(Viewshed { range: 9, visible: Vec::new(), refresh: true }),
        members: vec![
            PartyMember {
                name: make_beast_name(rng),
                class: String::from("Beast"),
                icon: Render::new(98, ColorPair::new(RED,BLACK), 255),
                abilities: vec![],
                health: Health::new(30 + hp_mod),
                attack: Attack::new(4,4),
                threat: Threat::new(12, 10),
                modifiers: vec![Modifier::new(ModifierEffect::Block(2), 0, true)],
            }
        ],
        ai: Some(AIClass::new()),
        ..Default::default()
    }
}

pub fn spawn_elf_pickup(rng: &mut RandomNumberGenerator, pos: Point, f: u32) -> Object {
    let diceroll = rng.roll_dice(1, 7);
    let member = match diceroll {
        1 => vec![make_bard()],
        2 => vec![make_guardian()],
        3 => vec![make_barbarian()],
        4 => vec![make_woodcutter()],
        5 => vec![make_hunter()],
        6 => vec![make_cleric()],
        7 => vec![make_mage()],
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
        health: Health::new(35),
        attack: Attack::new(1,6),
        threat: Threat::new(6, 3),
        modifiers: Vec::new(),
    }
}
pub fn make_barbarian() -> PartyMember {
    PartyMember {
        name: format!("{}", make_random_elf_name()),
        class: String::from("Barbarian"),
        icon: Render::new(2, ColorPair::new(RED,BLACK), 255),
        abilities: vec![],
        health: Health::new(28),
        attack: Attack::new(2,6),
        threat: Threat::new(7, 2),
        modifiers: Vec::new(),
    }
}
pub fn make_woodcutter() -> PartyMember {
    PartyMember {
        name: format!("{}", make_random_elf_name()),
        class: String::from("Woodcutter"),
        icon: Render::new(2, ColorPair::new(DARK_GREEN,BLACK), 255),
        abilities: vec![AbilityClass::new(Ability::Deforest)],
        health: Health::new(18),
        attack: Attack::new(1,8),
        threat: Threat::new(2, 4),
        modifiers: Vec::new(),
    }
}
pub fn make_hunter() -> PartyMember {
    PartyMember {
        name: format!("{}", make_random_elf_name()),
        class: String::from("Hunter"),
        icon: Render::new(2, ColorPair::new(SEA_GREEN,BLACK), 255),
        abilities: vec![AbilityClass::new(Ability::KillShot)],
        health: Health::new(16),
        attack: Attack::new(1,6),
        threat: Threat::new(1, 3),
        modifiers: Vec::new(),
    }
}
pub fn make_cleric() -> PartyMember {
    PartyMember {
        name: format!("{}", make_random_elf_name()),
        class: String::from("Cleric"),
        icon: Render::new(2, ColorPair::new(WHITE,BLACK), 255),
        abilities: vec![AbilityClass::new(Ability::CureWounds)],
        health: Health::new(10),
        attack: Attack::new(1,3),
        threat: Threat::new(8, 0),
        modifiers: Vec::new(),
    }
}
pub fn make_mage() -> PartyMember {
    PartyMember {
        name: format!("{}", make_random_elf_name()),
        class: String::from("Mage"),
        icon: Render::new(2, ColorPair::new(CYAN,BLACK), 255),
        abilities: vec![AbilityClass::new(Ability::MagicMissile)],
        health: Health::new(12),
        attack: Attack::new(1,4),
        threat: Threat::new(2, 2),
        modifiers: Vec::new(),
    }
}

//Enemy member definitions
pub fn enemy_make_forsaken_warrior(rng: &mut RandomNumberGenerator, f: u32) -> PartyMember {
    let hp_mod = rng.range(1, f as i32 + 3);
    PartyMember {
        name: String::from("Forsaken Elf"),
        class: String::from("Warrior"),
        icon: Render::new(1, ColorPair::new(PURPLE,BLACK), 255),
        abilities: vec![],
        health: Health::new(10 + hp_mod),
        attack: Attack::new(1,rng.range(4, min(4 + (f / 2) as i32, 8) + 1)),
        threat: Threat::new(4, 2),
        modifiers: Vec::new(),
    }
}
pub fn enemy_make_forsaken_caster(rng: &mut RandomNumberGenerator, f: u32) -> PartyMember {
    let hp_mod = rng.range(1, f as i32 + 3);
    PartyMember {
        name: String::from("Dark Magus"),
        class: String::from("Psychomancer"),
        icon: Render::new(1, ColorPair::new(BLUE_VIOLET,BLACK), 255),
        abilities: vec![AbilityClass::new(Ability::PsyBolt)],
        health: Health::new(5 + hp_mod),
        attack: Attack::new(1,rng.range(2, min(2 + (f / 2) as i32, 4) + 1)),
        threat: Threat::new(2, 0),
        modifiers: Vec::new(),
    }
}


fn make_beast_name(rng: &mut RandomNumberGenerator) -> String {
    let rand1 = rng.roll_dice(1, 101);
    let rand2 = rng.roll_dice(1, 101);

    let string1 =
        if rand1 >= 1 && rand1 <= 10 { "Ik" }
        else if rand1 >= 11 && rand1 <= 20 { "Mul" }
        else if rand1 >= 21 && rand1 <= 30 { "Kar" }
        else if rand1 >= 31 && rand1 <= 40 { "Than" }
        else if rand1 >= 41 && rand1 <= 50 { "Rom" }
        else if rand1 >= 51 && rand1 <= 60 { "Ga" }
        else if rand1 >= 61 && rand1 <= 70 { "Ki" }
        else if rand1 >= 71 && rand1 <= 80 { "Nar" }
        else if rand1 >= 81 && rand1 <= 90 { "Gen" }
        else if rand1 >= 91 && rand1 <= 100 { "Att" }
        else {"!!!"};

    let string2 =
        if rand2 >= 1 && rand2 <= 10 { "tar" }
        else if rand2 >= 11 && rand2 <= 20 { "dros" }
        else if rand2 >= 21 && rand2 <= 30 { "gor" }
        else if rand2 >= 31 && rand2 <= 40 { "har" }
        else if rand2 >= 41 && rand2 <= 50 { "kem" }
        else if rand2 >= 51 && rand2 <= 60 { "gol" }
        else if rand2 >= 61 && rand2 <= 70 { "ra" }
        else if rand2 >= 71 && rand2 <= 80 { "tan" }
        else if rand2 >= 81 && rand2 <= 90 { "vek" }
        else if rand2 >= 91 && rand2 <= 100 { "kas" }
        else {"!!!"};

    return String::from(format!("{}{}", string1, string2))
}