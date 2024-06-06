fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input file");
    let containers = parse_input(&input);
    println!("Answer #1 is {}", count_combinations(&containers, 150));
    println!("Answer #2 is {}", count_minimum_combinations(&containers, 150));
}

fn parse_input(input: &str) -> Vec<i32> {
    let mut c: Vec<i32> = input.trim().lines().map(|s| s.parse::<i32>().unwrap()).collect();
    c.sort_by(|a, b| b.cmp(a));
    c
}

fn enumerate_combinations_rec<F>(cs: &mut [i32], csindex: usize, containers: &[i32], index: usize, amount: i32, func: &mut F) where F: FnMut(&[i32]) {
    if index >= cs.len() { return; }
    let current_amount = containers[index];
    enumerate_combinations_rec(cs, csindex, containers, index + 1, amount, func);
    cs[csindex] = containers[index];
    if current_amount == amount {
        func(&cs[0..csindex + 1]);
    } else if current_amount < amount {
        enumerate_combinations_rec(cs, csindex + 1, containers, index + 1, amount - current_amount, func);
    }
}

fn enumerate_combinations<F>(containers: &[i32], amount: i32, mut func: F) where F: FnMut(&[i32]) {
    let mut cs = vec![0; containers.len()];
    enumerate_combinations_rec(&mut cs, 0, containers, 0, amount, &mut func);
}

fn count_combinations(containers: &[i32], amount: i32) -> i32 {
    let mut count = 0;
    enumerate_combinations(containers, amount, |_| {count += 1;});
    count
}

fn count_minimum_combinations(containers: &[i32], amount: i32) -> i32 {
    let mut counts = std::collections::HashMap::<usize, i32>::new();
    let mut min_key = containers.len();
    enumerate_combinations(containers, amount, |c| {
        min_key = std::cmp::min(min_key, c.len());
        *counts.entry(c.len()).or_insert(0) += 1;
    });
    *counts.get(&min_key).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "20\n15\n10\n5\n5\n"
    }

    #[test]
    fn test_parse_input() {
        let containers = parse_input(&sample_input());
        assert_eq!(containers, vec![20, 15, 10, 5, 5]);
    }

    #[test]
    fn test_enumerate_combinations() {
        let mut count = 0;
        enumerate_combinations(&parse_input(&sample_input()), 25, |c| {
            match count {
                0 => { assert_eq!(c, vec![15, 5, 5]); },
                1 => { assert_eq!(c, vec![15, 10]); },
                2 => { assert_eq!(c, vec![20, 5]); },
                3 => { assert_eq!(c, vec![20, 5]); },
                _ => panic!("Unexpected combination #{} - {:?}", count, c)
            }
            count += 1;
        });
        assert_eq!(count, 4);
    }

    #[test]
    fn test_count_combinations() {
        assert_eq!(count_combinations(&parse_input(&sample_input()), 25), 4);
    }

    #[test]
    fn test_count_minimum_combinations() {
        assert_eq!(count_minimum_combinations(&parse_input(&sample_input()), 25), 3);
    }
}