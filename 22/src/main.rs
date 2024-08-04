use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").expect("Unable to read input file");
    let boss = parse_input(&input);
    println!("Answer #1 is {}", find_cheapest_game(&PlayerCharacter::new(), &boss, false));
    println!("Answer #2 is {}", find_cheapest_game(&PlayerCharacter::new(), &boss, true));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct BossCharacter {
    hit_points: i32,
    damage: i32,
}

fn parse_input(input: &str) -> BossCharacter {
    let mut lines = input.lines();
    let hit_points = lines.nth(0).unwrap().split(":").nth(1).unwrap().trim().parse::<i32>().unwrap();
    let damage = lines.nth(0).unwrap().split(":").nth(1).unwrap().trim().parse::<i32>().unwrap();
    BossCharacter { hit_points, damage }
}

#[derive(Clone)]
struct Spell {
    _name: String,
    mana_cost: i32,
    damage: i32,
    heal: i32,
    effect: Effect,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Effect {
    None,
    Shield,
    Poison,
    Recharge,
}

impl Effect {
    fn duration(&self) -> i32 {
        match self {
            Effect::None => 0,
            Effect::Shield => 6,
            Effect::Poison => 6,
            Effect::Recharge => 5,
        }
    }
}

/*
    Magic Missile costs 53 mana. It instantly does 4 damage.
    Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
    Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
    Poison costs 173 mana. It starts an effect that lasts for 6 turns. 
    At the start of each turn while it is active, it deals the boss 3 damage.
    Recharge costs 229 mana. It starts an effect that lasts for 5 turns.
    At the start of each turn while it is active, it gives you 101 new mana.
*/
fn all_spells() -> Vec<Spell> {
    vec![
        Spell { _name: "Magic Missile".into(), mana_cost: 53, damage: 4, heal: 0, effect: Effect::None },
        Spell { _name: "Drain".into(), mana_cost: 73, damage: 2, heal: 2, effect: Effect::None },
        Spell { _name: "Shield".into(), mana_cost: 113, damage: 0, heal: 0, effect: Effect::Shield },
        Spell { _name: "Poison".into(), mana_cost: 173, damage: 0, heal: 0, effect: Effect::Poison },
        Spell { _name: "Recharge".into(), mana_cost: 229, damage: 0, heal: 0, effect: Effect::Recharge },
    ]
}

#[derive(Clone)]
struct ActiveEffects {
    active: HashMap<Effect, i32>,
}

impl ActiveEffects {
    fn new() -> Self {
        Self { active: HashMap::new() }
    }

    fn is_active(&self, effect: Effect) -> bool {
        self.active.contains_key(&effect)
    }

    fn is_valid_to_cast(&self, effect: Effect) -> bool {
        let a = self.active.get(&effect);
        match a {
            None => true,
            Some(&n) => n == 1,
        }
    }

    fn next_turn(&mut self) {
        self.active.retain(|_k, v| {
            *v -= 1;
            *v != 0
         });
    }

    fn add_effect(&mut self, effect: Effect) {
        if effect == Effect::None { return; }
        let duration = effect.duration();
        let r = self.active.insert(effect, duration);
        assert!(r.is_none());
    }
}

#[derive(Clone)]
struct PlayerCharacter {
    hit_points: i32,
    mana: i32,
    active_effects: ActiveEffects,
}

impl PlayerCharacter {
    fn new() -> Self {
        Self { hit_points: 50, mana: 500, active_effects: ActiveEffects::new() }
    }
}

fn play_turn(player: &PlayerCharacter, boss: &BossCharacter, action: &Spell, hard_mode: bool) -> (PlayerCharacter, BossCharacter) {
    let mut ret_player = (*player).clone();
    let mut ret_boss = *boss;
    #[derive(PartialEq, Eq)] enum Turn { Player, Boss }
    for turn in [Turn::Player, Turn::Boss] {
        if hard_mode && turn == Turn::Player {
            ret_player.hit_points -= 1;
            if ret_player.hit_points <= 0 {
                break;
            }
        }
        if ret_player.active_effects.is_active(Effect::Poison) { ret_boss.hit_points -= 3; }
        if ret_player.active_effects.is_active(Effect::Recharge) { ret_player.mana += 101; }
        ret_player.active_effects.next_turn();
        match turn {
            Turn::Player => {
                ret_player.active_effects.add_effect(action.effect);
                ret_boss.hit_points -= action.damage;
                ret_player.hit_points += action.heal;
                ret_player.mana -= action.mana_cost;
            },
            Turn::Boss => {
                if ret_boss.hit_points > 0 {
                    let is_shielded = ret_player.active_effects.is_active(Effect::Shield);
                    let boss_damage = ret_boss.damage - if is_shielded { 7 } else { 0 };
                    ret_player.hit_points -= std::cmp::max(1, boss_damage);
                }
            },
        }
    }
    (ret_player, ret_boss)
}

fn is_valid_action(player: &PlayerCharacter, action: &Spell) -> bool {
    action.mana_cost <= player.mana && player.active_effects.is_valid_to_cast(action.effect)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GameResult {
    Pending,
    PlayerWins,
    BossWins,
    Invalid,
}

fn play_game(player: &PlayerCharacter, boss: &BossCharacter, turns: &[usize], hard_mode: bool) -> GameResult {
    let spells = all_spells();
    let mut player = (*player).clone();
    let mut boss = *boss;
    for &turn in turns {
        let action = &spells[turn];
        if !is_valid_action(&player, action) { return GameResult::Invalid; }
        (player, boss) = play_turn(&player, &boss, action, hard_mode);
        if player.hit_points <= 0 { return GameResult::BossWins; }
        if boss.hit_points <= 0 { return GameResult::PlayerWins; }
    }
    GameResult::Pending
}

fn mana_cost_for_game(turns: &[usize]) -> i32 {
    let spells = all_spells();
    turns.iter().fold(0, |acc, turn| {
        let action = &spells[*turn];
        acc + action.mana_cost
    })
}

fn find_cheapest_game(player: &PlayerCharacter, boss: &BossCharacter, hard_mode: bool) -> i32 {
    let mut turns = Vec::new();
    find_cheapest_game_rec(player, boss, &mut turns, i32::MAX, hard_mode)
}

fn find_cheapest_game_rec(player: &PlayerCharacter, boss: &BossCharacter, turns: &mut Vec<usize>, mut current_min: i32, hard_mode: bool) -> i32 {
    // DFS: prune as soon as we use more mana than current minimum
    let n_spells = 5;
    let cheapest_spell = 53;
    turns.push(0);
    while *turns.last().unwrap() < n_spells {
        match play_game(player, boss, turns, hard_mode) {
            GameResult::PlayerWins => {
                current_min = std::cmp::min(current_min, mana_cost_for_game(turns));
            },
            GameResult::Pending =>  {
                if mana_cost_for_game(turns) + cheapest_spell < current_min {
                    current_min = find_cheapest_game_rec(player, boss, turns, current_min, hard_mode);
                }
            },
            _ => {},
        }
        *turns.last_mut().unwrap() += 1;
    }
    turns.pop();
    current_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_1() {
        let player = PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() };
        let boss = BossCharacter { hit_points: 13, damage: 8 };
        let spells = all_spells();
        assert!(!player.active_effects.is_active(Effect::Poison));
        assert_eq!(player.hit_points, 10);
        assert_eq!(player.mana, 250);
        assert_eq!(boss, BossCharacter{ hit_points: 13, damage: 8 });
        
        let (player, boss) = play_turn(&player, &boss, &spells[3], false);
        assert!(player.active_effects.is_active(Effect::Poison));
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.mana, 77);
        assert_eq!(boss, BossCharacter{ hit_points: 10, damage: 8 });

        let (player, boss) = play_turn(&player, &boss, &spells[0], false);
        assert!(player.active_effects.is_active(Effect::Poison));
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.mana, 24);
        assert_eq!(boss, BossCharacter{ hit_points: 0, damage: 8 });
    }

    #[test]
    fn test_game_2() {
        let player = PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() };
        let boss = BossCharacter { hit_points: 14, damage: 8 };
        let spells = all_spells();
        assert!(!player.active_effects.is_active(Effect::Recharge));
        assert_eq!(player.hit_points, 10);
        assert_eq!(player.mana, 250);
        assert_eq!(boss, BossCharacter{ hit_points: 14, damage: 8 });

        // cast recharge
        let (player, boss) = play_turn(&player, &boss, &spells[4], false);
        assert!(player.active_effects.is_active(Effect::Recharge));
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.mana, 122);
        assert_eq!(boss, BossCharacter{ hit_points: 14, damage: 8 });

        // cast shield
        let (player, boss) = play_turn(&player, &boss, &spells[2], false);
        assert!(player.active_effects.is_active(Effect::Recharge));
        assert!(player.active_effects.is_active(Effect::Shield));
        assert_eq!(player.hit_points, 1);
        assert_eq!(player.mana, 211);
        assert_eq!(boss, BossCharacter{ hit_points: 14, damage: 8 });

        // cast drain
        let (player, boss) = play_turn(&player, &boss, &spells[1], false);
        assert!(player.active_effects.is_active(Effect::Shield));
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.mana, 340);
        assert_eq!(boss, BossCharacter{ hit_points: 12, damage: 8 });

        // cast poison
        let (player, boss) = play_turn(&player, &boss, &spells[3], false);
        assert!(player.active_effects.is_active(Effect::Shield));
        assert!(player.active_effects.is_active(Effect::Poison));
        assert_eq!(player.hit_points, 1);
        assert_eq!(player.mana, 167);
        assert_eq!(boss, BossCharacter{ hit_points: 9, damage: 8 });

        // cast missile
        let (player, boss) = play_turn(&player, &boss, &spells[0], false);
        assert!(player.active_effects.is_active(Effect::Poison));
        assert_eq!(player.hit_points, 1);
        assert_eq!(player.mana, 114);
        assert_eq!(boss, BossCharacter{ hit_points: -1, damage: 8 });
    }

    #[test]
    fn test_play_game() {
        assert_eq!(play_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                             &BossCharacter { hit_points: 13, damage: 8 },
                             &[3], false), GameResult::Pending);
        assert_eq!(play_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                             &BossCharacter { hit_points: 13, damage: 8 },
                             &[3, 0], false), GameResult::PlayerWins);
        assert_eq!(play_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                             &BossCharacter { hit_points: 13, damage: 8 },
                             &[3, 3], false), GameResult::Invalid);
        assert_eq!(play_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                             &BossCharacter { hit_points: 13, damage: 8 },
                             &[3, 1], false), GameResult::BossWins);

        assert_eq!(play_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                             &BossCharacter { hit_points: 14, damage: 8 },
                             &[3, 0], false), GameResult::BossWins);
        assert_eq!(play_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                             &BossCharacter { hit_points: 14, damage: 8 },
                             &[4, 2, 1, 3], false), GameResult::Pending);
        assert_eq!(play_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                             &BossCharacter { hit_points: 14, damage: 8 },
                             &[4, 2, 1, 3, 0], false), GameResult::PlayerWins);
        assert_eq!(play_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                             &BossCharacter { hit_points: 14, damage: 8 },
                             &[4, 2, 1, 3, 3], false), GameResult::Invalid);
        assert_eq!(play_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                             &BossCharacter { hit_points: 14, damage: 8 },
                             &[4, 2, 1, 3, 1], false), GameResult::BossWins);
        assert_eq!(play_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                             &BossCharacter { hit_points: 14, damage: 8 },
                             &[2, 2, 2, 2], false), GameResult::Invalid);
    }

    #[test]
    fn test_mana_cost() {
        assert_eq!(mana_cost_for_game(&[3, 0]), 226);
        
        assert_eq!(mana_cost_for_game(&[4, 2, 1, 3, 0]), 641);
    }

    #[test]
    fn test_find_cheapest_game() {
        assert_eq!(find_cheapest_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                                      &BossCharacter { hit_points: 13, damage: 8 }, false), 226,);
        assert_eq!(find_cheapest_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                                      &BossCharacter { hit_points: 14, damage: 8 }, false), 641);

        assert_eq!(find_cheapest_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                                      &BossCharacter { hit_points: 13, damage: 8 }, true), i32::MAX);
        assert_eq!(find_cheapest_game(&PlayerCharacter { hit_points: 10, mana: 250, active_effects: ActiveEffects::new() },
                                      &BossCharacter { hit_points: 8, damage: 8 }, true), 219);
    }
}
