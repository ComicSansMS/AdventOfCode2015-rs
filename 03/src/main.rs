use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input file");

    println!("Answer #1 is {}", walk_the_map(&input).len());
    println!("Answer #2 is {}", walk_with_robo_santa(&input).len());
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn move_point(&mut self, c: char) {
        if c == '>' {
            self.x += 1;
        } else if c == '<' {
            self.x -= 1;
        } else if c == '^' {
            self.y += 1;
        } else if c == 'v' {
            self.y -= 1;
        } else {
            panic!("Invalid direction for move '{}'", c);
        }
    }
}

fn walk_the_map(input: &str) -> HashMap<Point, i32> {
    let current = Point { x: 0, y: 0 };
    let counts = HashMap::<Point, i32>::from([(current, 0)]);
    walk_the_map_impl(input, current, counts)
}

fn walk_the_map_impl(input: &str, mut current: Point, mut counts: HashMap<Point, i32>) -> HashMap<Point, i32> {
    for c in input.chars() {
        current.move_point(c);
        *counts.entry(current).or_insert(0) += 1;
    }
    counts
}

fn walk_with_robo_santa(input: &str) -> HashMap<Point, i32> {
    let m = walk_the_map(&input.chars().step_by(2).collect::<String>());
    walk_the_map_impl(&input.chars().skip(1).step_by(2).collect::<String>(), Point { x: 0, y: 0 }, m)
}

#[cfg(test)]
mod tests {
    use crate::{walk_the_map, walk_with_robo_santa, Point};
    #[test]
    fn test_move_point() {
        let mut p = Point{ x: 0, y: 0};
        p.move_point('^');
        assert_eq!(p, Point{ x: 0, y: 1 });
        p.move_point('>');
        assert_eq!(p, Point{ x: 1, y: 1 });
        p.move_point('v');
        assert_eq!(p, Point{ x: 1, y: 0 });
        p.move_point('<');
        assert_eq!(p, Point{ x: 0, y: 0 });
    }

    #[test]
    fn test_walk_the_map() {
        assert_eq!(walk_the_map(">").len(), 2);
        assert_eq!(walk_the_map("^>v<").len(), 4);
        assert_eq!(walk_the_map("^v^v^v^v^v").len(), 2);
    }

    #[test]
    fn test_walk_with_robo_santa() {
        assert_eq!(walk_with_robo_santa("^v").len(), 3);
        assert_eq!(walk_with_robo_santa("^>v<").len(), 3);
        assert_eq!(walk_with_robo_santa("^v^v^v^v^v").len(), 11);
    }
}