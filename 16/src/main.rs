use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input file");
    let aunts = parse_input(&input);
    println!("Answer #1 is {}", find_best_match(&aunts, &profile(), match_score) + 1);
    println!("Answer #2 is {}", find_best_match(&aunts, &profile(), match_score2) + 1);
}

fn profile() -> Aunt {
    let mut m = HashMap::<String, i64>::new();
    m.insert("children".into(), 3);
    m.insert("cats".into(), 7);
    m.insert("samoyeds".into(), 2);
    m.insert("pomeranians".into(), 3);
    m.insert("akitas".into(), 0);
    m.insert("vizslas".into(), 0);
    m.insert("goldfish".into(), 5);
    m.insert("trees".into(), 3);
    m.insert("cars".into(), 2);
    m.insert("perfumes".into(), 1);
    Aunt { things: m }
}

#[derive(Debug)]
struct Aunt {
    things: HashMap<String, i64>,
}

fn parse_input(input: &str) -> Vec<Aunt> {
    let mut v = Vec::new();
    let rx = regex::Regex::new(r"^Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)$").expect("Invalid regex");
    for line in input.lines() {
        if let Some(matches) = rx.captures(line) {
            let (_, [_, i1, n1, i2, n2, i3, n3]) = matches.extract();
            let mut m = HashMap::<String, i64>::new();
            m.insert(i1.into(), n1.parse().unwrap());
            m.insert(i2.into(), n2.parse().unwrap());
            m.insert(i3.into(), n3.parse().unwrap());
            v.push(Aunt { things: m });
        } else {
            panic!("Failure to match line {}", line);
        }
    }
    v
}

fn match_score(aunt: &Aunt, profile: &Aunt) -> i64 {
    let mut score = 0;
    for (k, &v) in &profile.things {
        if let Some(&x) = aunt.things.get(k) {
            if x == v { score += 1; }
        }
    }
    score
}

fn match_score2(aunt: &Aunt, profile: &Aunt) -> i64 {
    let mut score = 0;
    for (k, &v) in &profile.things {
        if let Some(&x) = aunt.things.get(k) {
            if k == "cats" || k == "trees" {
                if x > v { score += 1; }
            } else if k == "pomeranians" || k == "goldfish" {
                if x < v { score += 1; }
            } else if x == v { 
                score += 1;
            }
        }
    }
    score
}

fn find_best_match<Matcher>(aunts: &[Aunt], profile: &Aunt, func: Matcher) -> usize
    where Matcher: Fn(&Aunt, &Aunt) -> i64
{
    aunts.iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| { func(x, profile).cmp(&func(y, profile)) })
        .unwrap().0
}
