fn main() {
    let input_filename = String::from("input.txt");
    let input = std::fs::read_to_string(input_filename).expect("Error reading input file");

    println!("Answer #1 is {}", count_floors(&input));
    println!("Answer #1 is {}", trigger_basement(&input));
}

fn process_char(c: char, count: &mut i32) {
    if  c == '(' {
        *count += 1;
    } else if c == ')' {
        *count -= 1;
    } else {
        panic!("Encountered invalid character '{c}'");
    }
}

fn count_floors(input_str: &str) -> i32 {
    let mut count = 0;
    for c in input_str.chars() {
        process_char(c, &mut count);
    }
    count
}

fn trigger_basement(input_str: &str) -> i32 {
    let mut count = 0;
    for (i, c) in input_str.chars().enumerate() {
        process_char(c, &mut count);
        if count == -1 {
            return (i as i32) + 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::{count_floors, trigger_basement};
    #[test]
    fn test_count_floor() {
        assert_eq!(count_floors(""), 0);

        assert_eq!(count_floors("(())"), 0);
        assert_eq!(count_floors("()()"), 0);
        
        assert_eq!(count_floors("((("), 3);
        assert_eq!(count_floors("(()(()("), 3);
        assert_eq!(count_floors("))((((("), 3);
        
        assert_eq!(count_floors("())"), -1);
        assert_eq!(count_floors("))("), -1);
        
        assert_eq!(count_floors(")))"), -3);
        assert_eq!(count_floors(")())())"), -3);
    }

    #[test]
    fn test_trigger_basement() {
        assert_eq!(trigger_basement(""), -1);
        assert_eq!(trigger_basement(")"), 1);
        assert_eq!(trigger_basement("()())"), 5);
    }
}
