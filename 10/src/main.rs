fn main() {
    println!("Hello, world!");
    let input = std::fs::read_to_string("input").expect("Error reading input");
    let mut s: String = input.trim().to_string();
    for _ in 0..40 {
        s = look_and_say(&s);
    }
    println!("Answer #1 is {}", s.len());
    for _ in 0..10 {
        s = look_and_say(&s);
    }
    println!("Answer #2 is {}", s.len());
}

fn look_and_say(s: &str) -> String {
    let mut ret = String::new();
    let mut add_number = |n: u32, count: i32| {
        ret.push_str(&count.to_string());
        ret.push_str(&n.to_string());
    };
    let mut last_number: Option<u32> = None;
    let mut last_count: i32 = 0;
    for c in s.chars() {
        assert!(c.is_numeric());
        let i: u32 = c.to_digit(10).expect("Not a digit");
        if let Some(n) = last_number {
            if i == n {
                last_count += 1;
            } else {
                add_number(n, last_count);
                last_number = Some(i);
                last_count = 1;
            }
        } else {
            last_number = Some(i);
            last_count = 1;
        }
    }
    if let Some(n) = last_number {
        add_number(n, last_count);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say(""), "");
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}