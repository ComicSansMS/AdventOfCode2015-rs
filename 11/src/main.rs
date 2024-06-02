fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input");
    let answer1 = next_password_after(&input);
    println!("Answer #1 is {}", answer1);
    let answer2 = next_password_after(&answer1);
    println!("Answer #2 is {}", answer2);
}

fn contains_straight(s: &str) -> bool {
    let mut last_letter: char = '\0';
    let mut straight_count = 0;
    for c in s.chars() {
        if c as u8 != (last_letter as u8) + 1 {
            last_letter = c;
            straight_count = 1;
        } else {
            straight_count += 1;
            last_letter = c;
            if straight_count == 3 { return true; }
        }
    }
    false
}

fn contains_no_bad_letters(s: &str) -> bool {
    s.find(|c| c == 'i' || c == 'o' || c == 'l').is_none()
}

fn contains_pairs(s: &str) -> bool {
    let mut last_letter: char = '\0';
    let mut straight_count = 0;
    let mut pair_count = 0;
    for c in s.chars() {
        if c == last_letter {
            straight_count += 1;
            if straight_count % 2 == 0 { pair_count += 1; }
            if pair_count == 2 { return true; }
        } else {
            last_letter = c;
            straight_count = 1;
        }
    }
    false
}

fn is_valid_password(s: &str) -> bool {
    contains_no_bad_letters(s) && contains_straight(s) && contains_pairs(s)
}

fn next_password_after(s: &str) -> String {
    let mut v = Vec::from(s);
    loop {
        for c in v.iter_mut().rev() {
            if *c != b'z' {
                *c += 1;
                if *c == b'i' || *c == b'o' || *c == b'l' {
                    *c += 1;
                }
                assert!(*c >= b'a' && *c <= b'z');
                break;
            } else {
                *c = b'a';
            }
        }
        let r = String::from_utf8(v.clone()).unwrap();
        if is_valid_password(&r) {
            return r;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_straight() {
        assert_eq!(contains_straight(""), false);
        assert_eq!(contains_straight("abc"), true);
        assert_eq!(contains_straight("xabc"), true);
        assert_eq!(contains_straight("abcx"), true);
        assert_eq!(contains_straight("xabcx"), true);
        assert_eq!(contains_straight("xbcdx"), true);
        assert_eq!(contains_straight("xcdex"), true);
        assert_eq!(contains_straight("xxyzx"), true);
        assert_eq!(contains_straight("xabdx"), false);
        assert_eq!(contains_straight("hijklmmn"), true);
        assert_eq!(contains_straight("abbceffg"), false);
    }

    #[test]
    fn test_contains_pairs() {
        assert_eq!(contains_pairs(""), false);
        assert_eq!(contains_pairs("aa"), false);
        assert_eq!(contains_pairs("aaa"), false);
        assert_eq!(contains_pairs("aabb"), true);
        assert_eq!(contains_pairs("xaabb"), true);
        assert_eq!(contains_pairs("aabbx"), true);
        assert_eq!(contains_pairs("xaabbx"), true);
        assert_eq!(contains_pairs("xaaxbbx"), true);
        assert_eq!(contains_pairs("abbceffg"), true);
        assert_eq!(contains_pairs("abbcegjk"), false);
    }

    #[test]
    fn test_contains_no_bad_letters() {
        assert_eq!(contains_no_bad_letters(""), true);
        assert_eq!(contains_no_bad_letters("abcdefg"), true);
        assert_eq!(contains_no_bad_letters("i"), false);
        assert_eq!(contains_no_bad_letters("o"), false);
        assert_eq!(contains_no_bad_letters("l"), false);
        assert_eq!(contains_no_bad_letters("abcidef"), false);
        assert_eq!(contains_no_bad_letters("abcodef"), false);
        assert_eq!(contains_no_bad_letters("abcldef"), false);
        assert_eq!(contains_no_bad_letters("hijklmmn"), false);
    }

    #[test]
    fn test_is_valid_password() {
        assert_eq!(is_valid_password(""), false);
        assert_eq!(is_valid_password("hijklmmn"), false);
        assert_eq!(is_valid_password("abbceffg"), false);
        assert_eq!(is_valid_password("abbcegjk"), false);
        assert_eq!(is_valid_password("abcdffaa"), true);
        assert_eq!(is_valid_password("ghjaabcc"), true);
    }

    #[test]
    fn test_next_password_after() {
        assert_eq!(next_password_after("abcdefgh"), "abcdffaa");
    }
}
