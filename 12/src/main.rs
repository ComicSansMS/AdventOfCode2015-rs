use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").expect("Unable to read input");
    let json = parse_json(&input.trim());
    println!("Answer #1 is {}", sum_all_numbers(&json));
    println!("Answer #2 is {}", sum_all_except_red(&json));
}

#[derive(Debug, PartialEq, Eq)]
enum JsonObject {
    String(String),
    Number(i64),
    Object(HashMap<String, JsonObject>),
    Array(Vec<JsonObject>),
}

fn parse_number(s: &str) -> Option<(i64, &str)> {
    let i_end = s.find(|c: char| !(c.is_digit(10) || c == '-')).unwrap_or(s.len());
    if let Ok(i) = s[0..i_end].parse::<i64>() {
        Some((i, &s[i_end..]))
    } else {
        None
    }
}

fn parse_string(s: &str) -> (String, &str) {
    assert_eq!(s.chars().nth(0).unwrap(), '"');
    let s_end = s[1..].find('"').expect("Unterminated string") + 1;
    (String::from(&s[1..s_end]), &s[s_end + 1..])
}

fn parse_array(s: &str) -> (Vec<JsonObject>, &str) {
    let mut v = Vec::new();
    assert!(s.chars().nth(0) == Some('['));
    let mut it = &s[1..];
    if it.chars().nth(0) == Some(']') { return (v, &it[1..]); }
    loop {
        let (j, new_it) = parse_json_impl(it);
        v.push(j);
        it = &new_it[1..];
        if let Some(c) = new_it.chars().nth(0) {
            if c == ']' {
                // end of array
                break;
            } else if c != ',' {
                panic!("Expected ',' at {}", new_it);
            }
        } else {
            panic!("Unexpected end of array")
        }
    }
    (v, it)
}

fn parse_object(s: &str) -> (HashMap<String, JsonObject>, &str) {
    let mut m = HashMap::new();
    assert_eq!(s.chars().nth(0), Some('{'));
    let mut it = &s[1..];
    if it.chars().nth(0) == Some('}') { return (m, &it[1..]); }
    loop {
        assert_eq!(it.chars().nth(0), Some('"'));
        let (key, it_v) = parse_string(it);
        assert_eq!(it_v.chars().nth(0), Some(':'));
        it = &it_v[1..];
        let (val, it_next) = parse_json_impl(it);
        m.insert(key, val);
        it = &it_next[1..];
        if let Some(c) = it_next.chars().nth(0) {
            if c == '}' {
                // end of object
                break;
            } else if c != ',' {
                panic!("Expected ',' - found {}", it_next);
            }
        } else {
            panic!("Unexpected end of object");
        }
    }
    (m, it)
}

fn parse_json(s: &str) -> JsonObject {
    let (r, s) = parse_json_impl(s);
    assert!(s.is_empty());
    r
}

fn parse_json_impl(s: &str) -> (JsonObject, &str) {
    match s.chars().nth(0) {
        Some('{') => {
            let (o, rest) = parse_object(s);
            return (JsonObject::Object(o), rest);
        },
        Some('[') => {
            let (a, rest) = parse_array(s);
            return (JsonObject::Array(a), rest);
        },
        Some('"') => {
            let (s, rest) = parse_string(s);
            return (JsonObject::String(s), rest);
        },
        Some(c) if c.is_digit(10) || c == '-' => {
            let (i, rest) = parse_number(s).expect("Error parsing number");
            return (JsonObject::Number(i), rest);
        },
        Some(c) => panic!("Unexpected character {}", c),
        _ => panic!("Unexpected end of string"),
    }
}

fn sum_all_numbers(j: &JsonObject) -> i64 {
    match j {
        JsonObject::Number(n) => *n,
        JsonObject::String(_) => 0,
        JsonObject::Array(v) =>
            v.iter().fold(0, |acc, e| acc + sum_all_numbers(e)),
        JsonObject::Object(m) => 
            m.iter().fold(0, |acc, (_, v)| acc + sum_all_numbers(v)),
    }
}

fn object_has_red(obj: &HashMap<String, JsonObject>) -> bool {
    for (k, v) in obj {
        assert_ne!(k, "red");
        match v {
            JsonObject::String(s) => if s == "red" { return true; },
            _ => {},
        };
    }
    false
}

fn sum_all_except_red(j: &JsonObject) -> i64 {
    match j {
        JsonObject::Number(n) => *n,
        JsonObject::String(_) => 0,
        JsonObject::Array(v) =>
            v.iter().fold(0, |acc, e| acc + sum_all_except_red(e)),
        JsonObject::Object(m) => 
            if object_has_red(m) { 0 } else { m.iter().fold(0, |acc, (_, v)| acc + sum_all_except_red(v)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        assert_eq!(parse_json("5"), JsonObject::Number(5));
        assert_eq!(parse_json("-42"), JsonObject::Number(-42));
        assert_eq!(parse_json(r#""a""#), JsonObject::String("a".to_string()));
        assert_eq!(parse_json(r#""foo""#), JsonObject::String("foo".to_string()));
        assert_eq!(parse_json(r#""foo bar baz""#), JsonObject::String("foo bar baz".to_string()));
        assert_eq!(parse_json(r#"{"a":2,"b":4}"#), JsonObject::Object(
            HashMap::from([
                ("a".into(), JsonObject::Number(2)),
                ("b".into(), JsonObject::Number(4)),
            ])
        ));
        assert_eq!(parse_json(r#"[[[3]]]"#), JsonObject::Array(
            vec![
                JsonObject::Array(vec![
                    JsonObject::Array(vec![JsonObject::Number(3)]),
                ])],
        ));
        assert_eq!(parse_json(r#"{"a":{"b":4},"c":-1}"#), JsonObject::Object(
            HashMap::from([
                ("a".into(), JsonObject::Object(
                    HashMap::from([("b".into(), JsonObject::Number(4))])
                )),
                ("c".into(), JsonObject::Number(-1)),
            ])
        ));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number(""), None);
        assert_eq!(parse_number("5"), Some((5, "")));
        assert_eq!(parse_number("12"), Some((12, "")));
        assert_eq!(parse_number("-12"), Some((-12, "")));
        assert_eq!(parse_number("-12a"), Some((-12, "a")));
        assert_eq!(parse_number("-12aaa"), Some((-12, "aaa")));
        assert_eq!(parse_number("-a12"), None);
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(parse_string(r#""""#), ("".to_string(), ""));
        assert_eq!(parse_string(r#"""""#), ("".to_string(), "\""));
        assert_eq!(parse_string(r#""a"b"#), ("a".to_string(), "b"));
        assert_eq!(parse_string(r#""aaa bbb"xyz"#), ("aaa bbb".to_string(), "xyz"));
    }

    #[test]
    fn test_parse_array() {
        assert_eq!(parse_array("[]"), (vec![], ""));
        assert_eq!(parse_array("[1,2,3]"), (vec![JsonObject::Number(1), JsonObject::Number(2), JsonObject::Number(3)], ""));
        assert_eq!(parse_array(r#"[1,"foo",3]"#), (vec![JsonObject::Number(1), JsonObject::String("foo".to_string()), JsonObject::Number(3)], ""));
    }

    #[test]
    fn test_parse_object() {
        assert_eq!(parse_object(r#"{}"#), (HashMap::from([]), ""));
        assert_eq!(parse_object(r#"{"foo":1,"bar":2}"#), (HashMap::from([
            ("foo".into(), JsonObject::Number(1)),
            ("bar".into(), JsonObject::Number(2)),
        ]), ""));
        assert_eq!(parse_object(r#"{"foo":1,"bar":"baz"}"#), (HashMap::from([
            ("foo".into(), JsonObject::Number(1)),
            ("bar".into(), JsonObject::String("baz".into())),
        ]), ""));
        assert_eq!(parse_object(r#"{"foo":1,"bar":[2,3]}aaa"#), (HashMap::from([
            ("foo".into(), JsonObject::Number(1)),
            ("bar".into(), JsonObject::Array(vec![JsonObject::Number(2), JsonObject::Number(3)])),
        ]), "aaa"));
    }

    #[test]
    fn test_sum_all_numbers() {
        assert_eq!(sum_all_numbers(&parse_json(r#"[1,2,3]"#)), 6);
        assert_eq!(sum_all_numbers(&parse_json(r#"{"a":2,"b":4}"#)), 6);
        assert_eq!(sum_all_numbers(&parse_json(r#"[[[3]]]"#)), 3);
        assert_eq!(sum_all_numbers(&parse_json(r#"{"a":{"b":4},"c":-1}"#)), 3);
        assert_eq!(sum_all_numbers(&parse_json(r#"{"a":[-1,1]}"#)), 0);
        assert_eq!(sum_all_numbers(&parse_json(r#"[-1,{"a":1}]"#)), 0);
        assert_eq!(sum_all_numbers(&parse_json(r#"[]"#)), 0);
        assert_eq!(sum_all_numbers(&parse_json(r#"{}"#)), 0);
    }

    #[test]
    fn test_object_has_red() {
        assert_eq!(object_has_red(&HashMap::from([])), false);
        assert_eq!(object_has_red(&HashMap::from([
            ("a".into(), JsonObject::Number(1)),
            ("b".into(), JsonObject::String("blue".into())),
            ("c".into(), JsonObject::Array(vec![JsonObject::String("red".into())])),
        ])), false);
        assert_eq!(object_has_red(&HashMap::from([
            ("a".into(), JsonObject::Number(1)),
            ("b".into(), JsonObject::String("red".into())),
            ("c".into(), JsonObject::Array(vec![JsonObject::Number(1)])),
        ])), true);
    }

    #[test]
    fn test_sum_all_except_red() {
        assert_eq!(sum_all_except_red(&parse_json(r#"[1,2,3]"#)), 6);
        assert_eq!(sum_all_except_red(&parse_json(r#"[1,{"c":"red","b":2},3]"#)), 4);
        assert_eq!(sum_all_except_red(&parse_json(r#"{"d":"red","e":[1,2,3,4],"f":5}"#)), 0);
        assert_eq!(sum_all_except_red(&parse_json(r#"[1,"red",5]"#)), 6);
    }
}
