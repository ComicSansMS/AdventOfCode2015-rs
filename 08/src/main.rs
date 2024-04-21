fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input");
    let answer1 = answer1(&input);
    println!("Answer #1 is {}", answer1);
    let answer2 = answer2(&input);
    println!("Answer #2 is {}", answer2);
}

fn size_in_code(s: &str) -> usize {
    s.len()
}

#[derive(Debug, PartialEq)]
enum ParseState {
    Normal,
    InSlash,
    AwaitHex1,
    AwaitHex2,
}

fn resolve_escapes(s: &str) -> (String, usize) {
    let mut ret = String::new();
    let mut state = ParseState::Normal;
    let mut hex_buffer = 0;
    let mut count = 0;
    for (i, c) in s.chars().enumerate() {
        if (i == 0) || (i == s.len() - 1) {
            assert!(c == '\"');
        } else {
            match state {
                ParseState::Normal => {
                    if c == '\\' {
                        state = ParseState::InSlash;
                    } else {
                        ret.push(c);
                        count += 1;
                    }
                },
                ParseState::InSlash => {
                    match c {
                        '\\' => { ret.push('\\'); count += 1; state = ParseState::Normal; },
                        '\"' => { ret.push('\"'); count += 1; state = ParseState::Normal; },
                        'x' => { hex_buffer = 0; state = ParseState::AwaitHex1; },
                        c => { panic!("Invalid escape {}", c); },
                    }
                },
                ParseState::AwaitHex1 => {
                    hex_buffer += c.to_digit(16).expect(&format!("Invalid hex {}", c)) * 16;
                    state = ParseState::AwaitHex2;
                },
                ParseState::AwaitHex2 => {
                    hex_buffer += c.to_digit(16).expect(&format!("Invalid hex {}", c));
                    ret.push(char::from_u32(hex_buffer).expect(&format!("Hex escape sequence resolves to invalid char: {}", hex_buffer)));
                    count += 1;
                    state = ParseState::Normal;
                },
            }
        }
    }
    assert!(state == ParseState::Normal);
    (ret, count)
}

fn size_of_string(s: &str) -> usize {
    let (_, size) = resolve_escapes(s);
    size
}

fn size_of_escaped_string(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        match c {
            '\\' => { count += 2; },
            '\"' => { count += 2; },
            _ => { count += 1; },
        }
    }
    count + 2
}

fn answer1(input: &str) -> usize {
    let mut acc_string_sizes = 0;
    let mut acc_code_sizes = 0;
    for l in input.lines() {
        acc_string_sizes += size_of_string(l);
        acc_code_sizes += size_in_code(l);
    }
    acc_code_sizes - acc_string_sizes
}

fn answer2(input: &str) -> usize {
    let mut acc_escaped_size = 0;
    let mut acc_code_sizes = 0;
    for l in input.lines() {
        acc_escaped_size += size_of_escaped_string(l);
        acc_code_sizes += size_in_code(l);
    }
    acc_escaped_size - acc_code_sizes
}

#[cfg(test)]
mod tests {
    use std::ops::ShlAssign;

    use crate::*;

    fn test_string() -> &'static str {
        r#"""
"abc"
"aaa\"aaa"
"\x27""#
    }

    #[test]
    fn test() {
        let quotes = r#""""#;
        let abc = r#""abc""#;
        let slash_quote = r#""aaa\"aaa""#;
        let slash_x = r#""\x27""#;
        let slash_slash = r#""a\\b""#;
        assert_eq!(size_in_code(quotes), 2);
        assert_eq!(size_in_code(abc), 5);
        assert_eq!(size_in_code(slash_quote), 10);
        assert_eq!(size_in_code(slash_x), 6);
        assert_eq!(size_in_code(slash_slash), 6);

        assert_eq!(size_of_string(quotes), 0);
        assert_eq!(size_of_string(abc), 3);
        assert_eq!(size_of_string(slash_quote), 7);
        assert_eq!(size_of_string(slash_x), 1);
        assert_eq!(size_of_string(slash_slash), 3);

        assert_eq!(size_of_escaped_string(quotes), 6);
        assert_eq!(size_of_escaped_string(abc), 9);
        assert_eq!(size_of_escaped_string(slash_quote), 16);
        assert_eq!(size_of_escaped_string(slash_x), 11);
        assert_eq!(size_of_escaped_string(slash_slash), 12);
    }

    #[test]
    fn test_answer1() {
        assert_eq!(answer1(test_string()), 12);
    }

    #[test]
    fn test_answer2() {
        assert_eq!(answer2(test_string()), 19);        
    }
}