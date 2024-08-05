fn main() {
    let input = std::fs::read_to_string("input").expect("Unable to read input");
    let (row, col) = parse_input(&input);
    println!("Answer is {}", find_code(row, col));
}

fn parse_input(input: &str) -> (i64, i64) {
    assert!(input.starts_with("To continue, please consult the code grid in the manual.  Enter the code at row "));
    (input[80..84].parse().unwrap(), input[93..97].parse().unwrap())
}

fn cantor_pairing(row: i64, col: i64) -> i64 {
    let row = row - 1;
    let col = col - 1;
    col + ((row + col)*(row + col + 1)) / 2 + 1
}

fn find_code(row: i64, col: i64) -> i64 {
    let limit = cantor_pairing(row, col) - 1;
    let factor = 252533;
    let divisor = 33554393;
    let mut acc = 20151125;
    for _ in 0..limit {
        acc = (acc * factor) % divisor;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantor_pairing() {
        assert_eq!(cantor_pairing(1, 1), 1);
        assert_eq!(cantor_pairing(2, 1), 2);
        assert_eq!(cantor_pairing(1, 2), 3);
        assert_eq!(cantor_pairing(3, 1), 4);
        assert_eq!(cantor_pairing(2, 2), 5);
        assert_eq!(cantor_pairing(6, 1), 16);
        assert_eq!(cantor_pairing(1, 5), 15);
        assert_eq!(cantor_pairing(3, 3), 13);
    }

    #[test]
    fn test_find_code() {
        assert_eq!(find_code(1, 1), 20151125);
        assert_eq!(find_code(1, 2), 18749137);
        assert_eq!(find_code(1, 3), 17289845);
        assert_eq!(find_code(1, 4), 30943339);
        assert_eq!(find_code(1, 5), 10071777);
        assert_eq!(find_code(4, 2), 32451966);
        assert_eq!(find_code(6, 6), 27995004);
    }
}
