use itertools::MinMaxResult;

fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input file");
    let connections = parse_input(&input);
    let g = Graph::new(&connections);
    let (a1, a2) = g.all_paths();
    println!("Answer #1 is {}", a1);
    println!("Answer #2 is {}", a2);
}

struct Connection {
    from: String,
    to: String,
    distance: i64,
}

fn parse_input(input: &str) -> Vec<Connection> {
    let mut ret = Vec::new();
    for l in input.lines() {
        ret.push(parse_line(l));
    }
    ret
}

fn parse_line(l: &str) -> Connection {
    let rx_line = regex::Regex::new(r"^(\w+) to (\w+) = (\d+)$").expect("Invalid regex");
    if let Some(matches) = rx_line.captures(l) {
        let (_, [from, to, s_distance]) = matches.extract();
        let distance = s_distance.parse().expect("Invalid distance");
        Connection {
            from: String::from(from),
            to: String::from(to),
            distance,
        }
    } else {
        panic!("Unmatched line in input: {l}");
    }
}

#[derive(Debug, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
    distance: i64,
}

struct Graph {
    cities: Vec<String>,
    edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(connections: &Vec<Connection>) -> Graph {
        let mut g = Graph {
            cities: Vec::new(),
            edges: Vec::new(),
        };
        g.add_connections(connections);
        g
    }

    fn get_city_index(&mut self, city: &str) -> usize {
        match self.cities.iter().position(|c| c == city) {
            Some(idx) => idx,
            None => {
                self.cities.push(city.to_string());
                self.edges.push(Vec::new());
                self.cities.len() - 1
            }
        }
    }

    fn add_connection(&mut self, connection: &Connection) {
        let from = self.get_city_index(&connection.from);
        let to = self.get_city_index(&connection.to);
        let distance = connection.distance;
        self.edges[from].push(Edge { from, to, distance });
        self.edges[to].push(Edge {
            from: to,
            to: from,
            distance,
        });
    }

    fn add_connections(&mut self, connections: &Vec<Connection>) {
        for c in connections {
            self.add_connection(c);
        }
    }

    fn distance(&self, from: usize, to: usize) -> i64 {
        for e in &self.edges[from] {
            if e.to == to {
                return e.distance;
            }
        }
        panic!("No edge from {} to {}", from, to);
    }

    fn path_length(&self, path: &[(usize, &String)]) -> i64 {
        let mut current_city = None;
        let mut distance_travelled = 0;
        for (next_city, _) in path {
            match current_city {
                Some(c) => {
                    distance_travelled += self.distance(c, *next_city);
                    current_city = Some(*next_city);
                }
                None => current_city = Some(*next_city),
            }
        }
        distance_travelled
    }

    fn all_paths(&self) -> (i64, i64) {
        use itertools::Itertools;
        let mm = self
            .cities
            .iter()
            .enumerate()
            .permutations(self.cities.len())
            .map(|p| self.path_length(&p))
            .minmax();
        match mm {
            MinMaxResult::MinMax(min, max) => (min, max),
            MinMaxResult::OneElement(m) => (m, m),
            MinMaxResult::NoElements => (0, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn sample_input() -> &'static str {
        r"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"
    }

    fn sample_graph() -> Graph {
        Graph::new(&parse_input(&sample_input()))
    }

    #[test]
    fn test_parse_input() {
        let connections = parse_input(sample_input());
        assert_eq!(connections.len(), 3);
        assert_eq!(connections[0].from, "London");
        assert_eq!(connections[0].to, "Dublin");
        assert_eq!(connections[0].distance, 464);

        assert_eq!(connections[1].from, "London");
        assert_eq!(connections[1].to, "Belfast");
        assert_eq!(connections[1].distance, 518);

        assert_eq!(connections[2].from, "Dublin");
        assert_eq!(connections[2].to, "Belfast");
        assert_eq!(connections[2].distance, 141);
    }

    #[test]
    fn build_graph() {
        let g = sample_graph();
        assert_eq!(g.cities, vec!["London", "Dublin", "Belfast"]);
        assert_eq!(
            g.edges[0],
            vec![
                Edge {
                    from: 0,
                    to: 1,
                    distance: 464
                },
                Edge {
                    from: 0,
                    to: 2,
                    distance: 518
                },
            ]
        );
        assert_eq!(
            g.edges[1],
            vec![
                Edge {
                    from: 1,
                    to: 0,
                    distance: 464
                },
                Edge {
                    from: 1,
                    to: 2,
                    distance: 141
                },
            ]
        );
        assert_eq!(
            g.edges[2],
            vec![
                Edge {
                    from: 2,
                    to: 0,
                    distance: 518
                },
                Edge {
                    from: 2,
                    to: 1,
                    distance: 141
                },
            ]
        );
    }

    #[test]
    fn minimum_distance() {
        let g = sample_graph();
        let (shortest, longest) = g.all_paths();
        assert_eq!(shortest, 605);
        assert_eq!(longest, 982);
    }
}
