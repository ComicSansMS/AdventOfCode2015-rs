use std::usize;

fn main() {
    let input = std::fs::read_to_string("input").expect("Unable to read input");
    let weights = parse_input(&input);
    println!("Answer #1 is {}", find_smallest_group(&weights, 3));
    println!("Answer #2 is {}", find_smallest_group(&weights, 4));
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.trim().parse::<i32>().expect("Invalid weight {}")).collect()
}

fn find_group_rec(weights: &[i32], target: i32, nums: &mut Vec<i32>, smallest_known: &mut usize, results: &mut Vec<Vec<i32>>) {
    if target == 0 {
        *smallest_known = std::cmp::min(*smallest_known, nums.len());
        results.push(nums.clone());
    }
    for i in 0..weights.len() {
        let weights = &weights[..weights.len() - i];
        let first = *weights.last().unwrap();
        if first <= target {
            if nums.len() + 1 <= *smallest_known {
                nums.push(first);
                find_group_rec(&weights[..weights.len() - 1], target - first, nums, smallest_known, results);
                nums.pop();
            }
        }
    }
}

fn calculate_qe(group: &[i32]) -> i64 {
    group.iter().fold(1 as i64, |acc, &x| acc * (x as i64))
}

fn find_smallest_group(weights: &[i32], n_groups: i32) -> i64 {
    let acc_sum = weights.iter().fold(0, |acc, &x| acc + x);
    assert!(acc_sum % n_groups == 0);
    let target = acc_sum / n_groups;
    let mut nums = Vec::new();
    let mut smallest_known = usize::MAX;
    let mut group1 = Vec::new();
    find_group_rec(&weights, target, &mut nums,&mut smallest_known, &mut group1);
    group1.iter().map(|g| calculate_qe(&g)).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        r"1
        2
        3
        4
        5
        7
        8
        9
        10
        11
"
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(sample_input()), vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11]);
    }

    #[test]
    fn test_find_smallest_sum() {
        let weights = parse_input(&sample_input());
        assert_eq!(find_smallest_group(&weights, 3), 99);
        assert_eq!(find_smallest_group(&weights, 4), 44);
    }
}
