use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input");
    let (replacements, start) = parse_input(&input);
    let r = apply_all_replacements(&replacements, &start);
    println!("Answer #1 is {}", r.len());
    println!("Answer #2 is {}", find_replacement_chain(&replacements, &start));
}

#[derive(Debug, PartialEq, Eq)]
struct Replacement {
    input: String,
    output: String,
}

fn parse_input(input: &str) -> (Vec<Replacement>, String) {
    let mut replacements = Vec::new();
    let mut in_replacements = true;
    let mut start = String::new();
    for line in input.trim().lines() {
        if line.is_empty() {
            in_replacements = false;
        } else if in_replacements {
            let v: Vec<_> = line.split(" => ").collect();
            replacements.push(Replacement{ input: v[0].into(), output: v[1].into() });
        } else {
            assert!(start.is_empty());
            start = line.into();
        }
    }
    (replacements, start)
}

fn apply_all_replacements(replacements: &[Replacement], start: &str) -> HashSet<String> {
    let mut r = HashSet::new();
    for Replacement{input, output} in replacements {
        for (m_idx, _) in start.match_indices(input) {
            let mut new = start[0..m_idx].to_owned();
            new.push_str(&output);
            new.push_str(&start[(m_idx+input.len())..]);
            r.insert(new);
        }
    }
    r
}

fn backward_replacements(replacements: &[Replacement], target: &str, working_set: &mut HashSet<String>) {
    for Replacement{input, output} in replacements {
        for (m_idx, _) in target.match_indices(output) {
            let mut new = target[0..m_idx].to_owned();
            new.push_str(input);
            new.push_str(&target[(m_idx+output.len())..]);
            working_set.insert(new);
        }
    }
}

fn find_replacement_chain(replacements: &[Replacement], target: &str) -> i32 {
    let mut words = Vec::<(String, i32)>::new();
    words.push((target.into(), 0));
    loop {
        words.sort_by(|(a, _), (b, _)| { b.len().cmp(&a.len()) });
        let (first_word, word_count) = words.pop().unwrap().clone();
        if first_word == "e" {
            return word_count;
        }
        let mut new_set = HashSet::new();
        backward_replacements(replacements, &first_word, &mut new_set);
        for w in &new_set {
            words.push((w.clone(), word_count + 1));
        }
        if words.is_empty() { return -1; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        r"H => HO
H => OH
O => HH

HOH
"
    }

    fn sample_input2() -> &'static str {
        r"e => H
e => O
H => HO
H => OH
O => HH

e"
    }

    #[test]
    fn test_parse_input() {
        let (replacements, start) = parse_input(&sample_input());
        assert_eq!(replacements, vec![
            Replacement { input: "H".into(), output: "HO".into() },
            Replacement { input: "H".into(), output: "OH".into() },
            Replacement { input: "O".into(), output: "HH".into() },
        ]);
        assert_eq!(start, "HOH");
    }

    #[test]
    fn test_appply_all_replacements() {
        let (replacements, start) = parse_input(&sample_input());
        let r = apply_all_replacements(&replacements, &start);
        assert_eq!(r.len(), 4);
        assert!(r.contains("HOOH"));
        assert!(r.contains("HOHO"));
        assert!(r.contains("OHOH"));
        assert!(r.contains("HHHH"));
        assert_eq!(apply_all_replacements(&replacements, "HOHOHO").len(), 7);
    }

    #[test]
    fn test_find_replacement_chain() {
        let (replacements, _) = parse_input(&sample_input2());
        assert_eq!(find_replacement_chain(&replacements, "e"), 0);
        assert_eq!(find_replacement_chain(&replacements, "HOH"), 3);
        assert_eq!(find_replacement_chain(&replacements, "HOHOHO"), 6);
    }
}