use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input file");

    let answer1 = input.lines().filter(|w| is_nice(w)).count();
    println!("Answer #1 is {}", answer1);
    let answer2 = input.lines().filter(|w| is_nice2(w)).count();
    println!("Answer #2 is {}", answer2);
}

fn contains_three_vowels(word: &str) -> bool {
    word.chars().filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u').count() >= 3
}

fn contains_double_letter(word: &str) -> bool {
    let mut previous = '\0';
    for c in word.chars() {
        if previous != '\0' && c == previous {
            return true;
        }
        previous = c;
    }
    false
}

fn contains_blacklisted(word: &str) -> bool {
    //ab, cd, pq, or xy
    let mut previous = '\0';
    for c in word.chars() {
        if previous != '\0' {
            if previous == 'a' && c == 'b' { return true; }
            if previous == 'c' && c == 'd' { return true; }
            if previous == 'p' && c == 'q' { return true; }
            if previous == 'x' && c == 'y' { return true; }
        }
        previous = c;
    }
    false
}

fn is_nice(word: &str) -> bool {
    contains_three_vowels(word) && contains_double_letter(word) && !contains_blacklisted(word)
}

fn contains_double_pair(word: &str) -> bool {
    let mut m = HashMap::<(char, char), usize>::new();
    let mut previous = '\0';
    for (i, c) in word.char_indices() {
        let pair = (previous, c);
        let e = m.get(&pair);
        if e.is_some_and(|pos| i - pos > 1) {
            return true;
        } else if e.is_none() {
            m.insert(pair, i);
        }
        previous = c;
    }
    false
}

fn contains_repeating_with_one_letter_between(word:&str) -> bool {
    let mut previous = '\0';
    let mut prev_previous = '\0';
    for c in word.chars() {
        if c == prev_previous { return true; }
        prev_previous = previous;
        previous = c;
    }
    false
}

fn is_nice2(word: &str) -> bool {
    contains_double_pair(word) && contains_repeating_with_one_letter_between(word)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_contains_three_vowels() {
        assert!(!contains_three_vowels("abcdef"));
        assert!(contains_three_vowels("aaa"));
        assert!(contains_three_vowels("eiu"));
        assert!(contains_three_vowels("volominous"));
    }

    #[test]
    fn test_contains_double_letter() {
        assert!(!contains_double_letter("abcd"));
        assert!(contains_double_letter("aa"));
        assert!(contains_double_letter("abba"));
        assert!(contains_double_letter("xxx"));
        assert!(!contains_double_letter("abab"));
    }

    #[test]
    fn test_contains_blacklisted() {
        assert!(!contains_blacklisted("word"));
        assert!(contains_blacklisted("rabcage"));
        assert!(contains_blacklisted("cd"));
        assert!(contains_blacklisted("pq-formel"));
        assert!(contains_blacklisted("aaaaaxy"));
    }

    #[test]
    fn test_is_nice() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_contains_double_pair() {
        assert!(contains_double_pair("xyxy"));
        assert!(contains_double_pair("aabcdefgaa"));
        assert!(!contains_double_pair("aaa"));
        assert!(contains_double_pair("aaaa"));
    }

    #[test]
    fn test_contains_repeating_with_one_letter_between() {
        assert!(!contains_repeating_with_one_letter_between("abcda"));
        assert!(contains_repeating_with_one_letter_between("xyx"));
        assert!(contains_repeating_with_one_letter_between("abcdefeghi"));
        assert!(contains_repeating_with_one_letter_between("aaa"));
        assert!(!contains_repeating_with_one_letter_between("aa"));
        assert!(contains_repeating_with_one_letter_between("aaaa"));
    }

    #[test]
    fn test_is_nice2() {
        assert!(!is_nice2("word"));
        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));
        assert!(!is_nice2("uurcxstgmygtbstg"));
        assert!(!is_nice2("ieodomkazucvgmuy"));
        assert!(is_nice2("aaaa"));
    }
}
