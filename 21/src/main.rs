fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input file");
    let boss = parse_input(&input);
    println!("Answer #1 is {}", cheapest_win(&boss));
    println!("Answer #2 is {}", most_expensive_loss(&boss));
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Character {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Weapon {
    name: String,
    cost: i32,
    damage: i32,
}

fn weapons() -> Vec<Weapon> {
    vec![
        Weapon { name: "Dagger".into(), cost: 8,  damage: 4 },
        Weapon { name: "Shortsword".into(), cost: 10,  damage: 5 },
        Weapon { name: "Warhammer".into(), cost: 25,  damage: 6 },
        Weapon { name: "Longsword".into(), cost: 40,  damage: 7 },
        Weapon { name: "Greataxe".into(), cost: 74,  damage: 8 },
    ]
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Armor {
    name: String,
    cost: i32,
    armor: i32,
}

fn armors() -> Vec<Armor> {
    vec![
        Armor { name: "Leather".into(), cost: 13, armor: 1 },
        Armor { name: "Chainmail".into(), cost: 31, armor: 2 },
        Armor { name: "Splintmail".into(), cost: 53, armor: 3 },
        Armor { name: "Bandedmail".into(), cost: 75, armor: 4 },
        Armor { name: "Platemail".into(), cost: 102, armor: 5 },
    ]
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Ring {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

fn rings() -> Vec<Ring> {
    vec![
        Ring { name: "Damage +1".into(), cost: 25, damage: 1, armor: 0 },
        Ring { name: "Damage +2".into(), cost: 50, damage: 2, armor: 0 },
        Ring { name: "Damage +3".into(), cost: 100, damage: 3, armor: 0 },
        Ring { name: "Defense +1".into(), cost: 20, damage: 0, armor: 1 },
        Ring { name: "Defense +2".into(), cost: 40, damage: 0, armor: 2 },
        Ring { name: "Defense +3".into(), cost: 80, damage: 0, armor: 3 },
    ]
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Inventory {
    weapon: Weapon,
    armor: Option<Armor>,
    ring1: Option<Ring>,
    ring2: Option<Ring>,
}

impl Inventory {
    fn cost(&self) -> i32 {
        self.weapon.cost +
        self.armor.as_ref().map_or(0, |a| a.cost) +
        self.ring1.as_ref().map_or(0, |r| r.cost) +
        self.ring2.as_ref().map_or(0, |r| r.cost)
    }

    fn damage(&self) -> i32 {
        self.weapon.damage +
        self.ring1.as_ref().map_or(0, |r| r.damage) +
        self.ring2.as_ref().map_or(0, |r| r.damage)
    }

    fn armor(&self) -> i32 {
        self.armor.as_ref().map_or(0, |a| a.armor) +
        self.ring1.as_ref().map_or(0, |r| r.armor) +
        self.ring2.as_ref().map_or(0, |r| r.armor)
    }
}

fn all_inventories() -> Vec<Inventory> {
    use itertools::Itertools;
    let mut r = Vec::new();
    let mut armors: Vec<_> = armors().into_iter().map(|a| Some(a)).collect();
    armors.push(None);
    let mut rings: Vec<_> = rings().into_iter().map(|r| Some(r)).collect();
    rings.push(None);
    let rings: Vec<_> = rings.clone().into_iter().cartesian_product(rings.into_iter())
        .filter(|(r1, r2)| {
            if r1.is_some() && r2.is_some() && r1.as_ref().unwrap() == r2.as_ref().unwrap() { return false; } else { return true; }
        })
        .collect();
    for w in weapons() {
        for a in &armors {
            for (r1, r2) in &rings {
                let inv = Inventory{ weapon: w.clone(), armor: a.clone(), ring1: r1.clone(), ring2: r2.clone() };
                r.push(inv);
            }
        }
    }
    r
}

fn player() -> Character {
    Character {
        hit_points: 100,
        damage: 0,
        armor: 0,
    }
}

fn player_equipped_with(inventory: &Inventory) -> Character {
    let mut p = player();
    p.damage += inventory.damage();
    p.armor += inventory.armor();
    p
}

fn parse_input(input: &str) -> Character {
    let mut lines = input.lines();
    let hit_points = lines.nth(0).unwrap().split(":").nth(1).unwrap().trim().parse::<i32>().unwrap();
    let damage = lines.nth(0).unwrap().split(":").nth(1).unwrap().trim().parse::<i32>().unwrap();
    let armor = lines.nth(0).unwrap().split(":").nth(1).unwrap().trim().parse::<i32>().unwrap();
    Character { hit_points, damage, armor }
}

fn damage_round(attacker: &Character, defender: &Character) -> i32 {
    let damage = std::cmp::max(attacker.damage - defender.armor, 1);
    damage
}

fn do_battle(boss: &Character, player: &Character) -> bool {
    let mut boss_hp = boss.hit_points;
    let mut player_hp = player.hit_points;
    loop {
        // Player attacks first
        boss_hp -= damage_round(player, boss);
        if boss_hp <= 0 { return true; }
        player_hp -= damage_round(boss, player);
        if player_hp <= 0 { return false; }
    }
}

fn cheapest_win(boss: &Character) -> i32 {
    all_inventories().iter()
        .filter(|inventory| {
            let player = player_equipped_with(&inventory);
            do_battle(boss, &player)
        })
        .map(|inventory| inventory.cost())
        .min()
        .unwrap()
}

fn most_expensive_loss(boss: &Character) -> i32 {
    all_inventories().iter()
        .filter(|inventory| {
            let player = player_equipped_with(&inventory);
            !do_battle(boss, &player)
        })
        .map(|inventory| inventory.cost())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        r"Hit Points: 12
Damage: 7
Armor: 2
"
    }

    #[test]
    fn test_parse_input() {
        let c = parse_input(&sample_input());
        assert_eq!(c.hit_points, 12);
        assert_eq!(c.damage, 7);
        assert_eq!(c.armor, 2);
    }

    #[test]
    fn test_damage_round() {
        assert_eq!(damage_round(&Character { hit_points: 100, damage: 8, armor: 0 },
                                &Character { hit_points: 100, damage: 0, armor: 3 }), 5);
        assert_eq!(damage_round(&Character { hit_points: 100, damage: 8, armor: 0 },
                                &Character { hit_points: 100, damage: 0, armor: 300 }), 1);
    }

    #[test]
    fn test_do_battle() {
        assert_eq!(do_battle(
            &Character { hit_points: 12, damage: 7, armor: 2 },
            &Character { hit_points: 8, damage: 5, armor: 5 }), true);
    }

    #[test]
    fn test_cost() {
        assert_eq!(Inventory{ weapon: Weapon{ cost: 10, name: "".into(), damage: 1 }, armor: None, ring1: None, ring2: None }.cost(), 10);
        assert_eq!(Inventory{ weapon: Weapon{ cost: 10, name: "".into(), damage: 1 },
                              armor: Some(Armor { cost: 7, name: "".into(), armor: 1 }), ring1: None, ring2: None }.cost(), 17);
        assert_eq!(Inventory{ weapon: Weapon{ cost: 10, name: "".into(), damage: 1 },
                              armor: Some(Armor { cost: 7, name: "".into(), armor: 1 }),
                              ring1: Some(Ring{ name: "".into(), cost: 8, damage: 1, armor: 1 }), ring2: None }.cost(), 25);
        assert_eq!(Inventory{ weapon: Weapon{ cost: 10, name: "".into(), damage: 1 },
                              armor: Some(Armor { cost: 7, name: "".into(), armor: 1 }),
                              ring1: Some(Ring{ name: "".into(), cost: 8, damage: 1, armor: 1 }),
                              ring2: Some(Ring{ name: "".into(), cost: 9, damage: 1, armor: 1 })
                            }.cost(), 34);
    }

    #[test]
    fn test_damage() {
        assert_eq!(Inventory{ weapon: Weapon{ cost: 10, name: "".into(), damage: 1 }, armor: None, ring1: None, ring2: None }.damage(), 1);
        assert_eq!(Inventory{ weapon: Weapon{ cost: 10, name: "".into(), damage: 1 },
                              armor: Some(Armor { cost: 7, name: "".into(), armor: 1 }), ring1: None, ring2: None }.damage(), 1);
        assert_eq!(Inventory{ weapon: Weapon{ cost: 10, name: "".into(), damage: 1 },
                              armor: Some(Armor { cost: 7, name: "".into(), armor: 1 }),
                              ring1: Some(Ring{ name: "".into(), cost: 8, damage: 9, armor: 1 }), ring2: None }.damage(), 10);
        assert_eq!(Inventory{ weapon: Weapon{ cost: 10, name: "".into(), damage: 1 },
                              armor: Some(Armor { cost: 7, name: "".into(), armor: 1 }),
                              ring1: Some(Ring{ name: "".into(), cost: 8, damage: 9, armor: 1 }),
                              ring2: Some(Ring{ name: "".into(), cost: 9, damage: 7, armor: 1 })
                            }.damage(), 17);
    }

    #[test]
    fn test_armor() {
        assert_eq!(Inventory{ weapon: Weapon{ cost: 10, name: "".into(), damage: 1 }, armor: None, ring1: None, ring2: None }.armor(), 0);
        assert_eq!(Inventory{ weapon: Weapon{ cost: 10, name: "".into(), damage: 1 },
                              armor: Some(Armor { cost: 7, name: "".into(), armor: 1 }), ring1: None, ring2: None }.armor(), 1);
        assert_eq!(Inventory{ weapon: Weapon{ cost: 10, name: "".into(), damage: 1 },
                              armor: Some(Armor { cost: 7, name: "".into(), armor: 1 }),
                              ring1: Some(Ring{ name: "".into(), cost: 8, damage: 1, armor: 9 }), ring2: None }.armor(), 10);
        assert_eq!(Inventory{ weapon: Weapon{ cost: 10, name: "".into(), damage: 1 },
                              armor: Some(Armor { cost: 7, name: "".into(), armor: 1 }),
                              ring1: Some(Ring{ name: "".into(), cost: 8, damage: 1, armor: 9 }),
                              ring2: Some(Ring{ name: "".into(), cost: 9, damage: 1, armor: 5 })
                            }.armor(), 15);
    }
}
