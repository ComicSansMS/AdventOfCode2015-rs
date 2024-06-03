use std::{collections::HashMap, hash::Hash};

fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input");
    let mut inp = parse_input(&input);
    println!("Answer #1 is {}", find_optimal_arrangement(&inp));
    inp.names.push("Me".into());
    println!("Answer #2 is {}", find_optimal_arrangement(&inp));
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Seat {
    person: usize,
    next_to: usize,
}

struct Input {
    names: Vec<String>,
    constraints: HashMap<Seat, i64>,
}

fn parse_input(s: &str) -> Input {
    let mut names = Vec::<String>::new();
    let mut names_map = HashMap::<String, usize>::new();
    let mut constraints = HashMap::new();
    let rx = regex::Regex::new(r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)\.$")
        .expect("Error parsing regex");
    for l in s.lines() {
        if let Some(matches) = rx.captures(l) {
            let(_, [name, gain_lose, happiness, neighbor]) = matches.extract();
            let name_id = *names_map.entry(name.into()).or_insert_with(|| { names.push(name.into()); names.len() - 1 });
            let neighbor_id = *names_map.entry(neighbor.into()).or_insert_with(|| { names.push(neighbor.into()); names.len() - 1 });
            let change_value = happiness.parse::<i64>().unwrap() * if gain_lose == "lose" { -1 } else { 1 };
            constraints.insert(Seat { person: name_id, next_to: neighbor_id }, change_value);
        } else {
            panic!("Invalid line format: {}", l);
        }
    }
    Input { names, constraints }
}

fn evaluate_arrangement(inp: &Input, arrangement: &[usize]) -> i64 {
    let mut next = arrangement[arrangement.len() - 1];
    let mut acc = 0;
    for &i in arrangement {
        if let Some(&v) = inp.constraints.get(&Seat{ person: i, next_to: next }) {
            acc += v;
        }
        if let Some(&v) = inp.constraints.get(&Seat{ person: next, next_to: i }) {
            acc += v;
        }
        next = i;
    }
    acc
}

fn find_optimal_arrangement(inp: &Input) -> i64 {
    use itertools::*;
    let n_guests = inp.names.len();
    let max_arrangement = (0..n_guests)
        .permutations(n_guests)
        .max_by_key(|arrangement| evaluate_arrangement(inp, &arrangement) );
    evaluate_arrangement(inp, &max_arrangement.unwrap())
}

#[cfg(test)]
mod tests{
    use super::*;

    fn sample_input() -> &'static str {
        return "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";
    }

    #[test]
    fn test_parse_input() {
        let input = parse_input(&sample_input());
        assert_eq!(input.names, vec!["Alice", "Bob", "Carol", "David"]);
        assert_eq!(input.constraints, HashMap::from([
            (Seat{ person: 0, next_to: 1 }, 54),
            (Seat{ person: 0, next_to: 2 }, -79),
            (Seat{ person: 0, next_to: 3 }, -2),
            (Seat{ person: 1, next_to: 0 }, 83),
            (Seat{ person: 1, next_to: 2 }, -7),
            (Seat{ person: 1, next_to: 3 }, -63),
            (Seat{ person: 2, next_to: 0 }, -62),
            (Seat{ person: 2, next_to: 1 }, 60),
            (Seat{ person: 2, next_to: 3 }, 55),
            (Seat{ person: 3, next_to: 0 }, 46),
            (Seat{ person: 3, next_to: 1 }, -7),
            (Seat{ person: 3, next_to: 2 }, 41),
        ]));
    }

    #[test]
    fn test_evaluate_arrangement() {
        let input = parse_input(&sample_input());
        let arrangement = vec![0, 1, 2, 3];
        assert_eq!(evaluate_arrangement(&input, &arrangement), 330);
    }

    #[test]
    fn test_find_optimal_arrangement() {
        let input = parse_input(&sample_input());
        assert_eq!(find_optimal_arrangement(&input), 330);
    }
}