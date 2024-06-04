fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input file");
    let reindeers = parse_input(&input);
    println!("Answer #1 is {}", winner_after(&reindeers, 2503));
    println!("Answer #1 is {}", new_score(&reindeers, 2503));
    //1089 too low
}

#[derive(Debug, PartialEq, Eq)]
struct Reindeer {
    name: String,
    speed: i64,
    travel_time: i64,
    rest_time: i64,
}

fn parse_input(s: &str) -> Vec<Reindeer> {
    let mut ret = Vec::new();
    let rx_line = regex::Regex::new(r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$").expect("Invalid regex");
    for l in s.trim().lines() {
        if let Some(matches) = rx_line.captures(l) {
            let (_, [name, speed, travel, rest]) = matches.extract();
            ret.push(Reindeer { name: name.into(), speed: speed.parse().unwrap(), travel_time: travel.parse().unwrap(), rest_time: rest.parse().unwrap()  })
        } else {
            panic!("Unable to match line {}", l);
        }
    }
    ret
}

fn calculate_position_at_time(r: &Reindeer, t: i64) -> i64 {
    let period = r.travel_time + r.rest_time;
    let full_distances = t / period;
    let remainder = t % period;
    let partial_distance = std::cmp::min(remainder, r.travel_time);
    (full_distances * r.travel_time + partial_distance) * r.speed
}

fn winner_after(rs: &[Reindeer], t: i64) -> i64 {
    rs.iter()
        .map(|r| calculate_position_at_time(r, t))
        .max()
        .unwrap()
}

fn new_score(rs: &[Reindeer], t: i64) -> i64 {
    let mut scoreboard = vec![0 as i64; rs.len()];
    for i in 0..t {
        let i = i + 1;
        let winning_score = winner_after(rs, i);
        for (idx, r) in scoreboard.iter_mut().enumerate() {
            if calculate_position_at_time(&rs[idx], i) == winning_score {
                *r += 1;
            }
        }
    }
    scoreboard.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
    "
    }

    #[test]
    fn test_parse_input() {
        let rs = parse_input(sample_input());
        assert_eq!(rs, vec![
            Reindeer { name: "Comet".into(), speed: 14, travel_time: 10, rest_time: 127 },
            Reindeer { name: "Dancer".into(), speed: 16, travel_time: 11, rest_time: 162 },
        ]);
    }

    #[test]
    fn test_calculate_position_at_time() {
        let rs = parse_input(&sample_input());
        assert_eq!(calculate_position_at_time(&rs[0], 1), 14);
        assert_eq!(calculate_position_at_time(&rs[1], 1), 16);
        assert_eq!(calculate_position_at_time(&rs[0], 10), 140);
        assert_eq!(calculate_position_at_time(&rs[1], 10), 160);
        assert_eq!(calculate_position_at_time(&rs[0], 11), 140);
        assert_eq!(calculate_position_at_time(&rs[1], 11), 176);
        assert_eq!(calculate_position_at_time(&rs[0], 12), 140);
        assert_eq!(calculate_position_at_time(&rs[1], 12), 176);
        assert_eq!(calculate_position_at_time(&rs[0], 138), 154);
        assert_eq!(calculate_position_at_time(&rs[1], 138), 176);
        assert_eq!(calculate_position_at_time(&rs[0], 174), 280);
        assert_eq!(calculate_position_at_time(&rs[1], 174), 192);
        assert_eq!(calculate_position_at_time(&rs[0], 1000), 1120);
        assert_eq!(calculate_position_at_time(&rs[1], 1000), 1056);
    }
    
    #[test]
    fn test_winner_after() {
        let rs = parse_input(&sample_input());
        assert_eq!(winner_after(&rs, 1000), 1120);
    }
    
    #[test]
    fn test_new_score() {
        let rs = parse_input(&sample_input());
        assert_eq!(new_score(&rs, 1000), 689);
    }
}
