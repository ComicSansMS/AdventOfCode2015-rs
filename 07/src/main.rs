use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input file");

    let mut commands = parse_input(&input);
    let wires = execute_program(&commands);
    println!("Answer #1 is {}", wires.value("a"));

    override_wires(&mut commands, wires.value("a"));
    let wires2 = execute_program(&commands);
    println!("Answer #2 is {}", wires2.value("a"));
}

fn override_wires(commands : &mut Vec<Command>, override_value: u16) {
    for c in commands {
        if c.destination == "b" {
            c.op = Operation::Assign(Operand::Number(override_value));
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operand {
    Number(u16),
    Register(String),
}

impl Operand {
    fn parse(str: &str) -> Operand {
        match str.parse::<u16>() {
            Ok(n) => Operand::Number(n),
            Err(_) => Operand::Register(String::from(str))
        }
    }

    fn resolve(&self, w: &Wires) -> u16 {
        match self {
            Operand::Number(n) => *n,
            Operand::Register(r) => w.value(r),
        }
    }

    fn is_ready(&self, w: &Wires) -> bool {
        match self {
            Operand::Number(_) => true,
            Operand::Register(r) => w.wire_has_value(r),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Assign(Operand),
    Not(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Lshift(Operand, Operand),
    Rshift(Operand, Operand),
}

impl Operation {
    fn execute(&self, wires: &Wires) -> u16 {
        match self {
            Operation::Assign(src) => {
                src.resolve(&wires)
            },
            Operation::Not(op) => {
                !op.resolve(&wires)
            },
            Operation::And(lhs, rhs) => {
                lhs.resolve(&wires) & rhs.resolve(&wires)
             },
            Operation::Or(lhs, rhs) => {
                lhs.resolve(&wires) | rhs.resolve(&wires)
             },
            Operation::Lshift(lhs, rhs) => {
                lhs.resolve(&wires) << rhs.resolve(&wires)
            },
            Operation::Rshift(lhs, rhs) => {
                lhs.resolve(&wires) >> rhs.resolve(&wires)
             },
        }
    }

    fn is_ready(&self, wires: &Wires) -> bool {
        match self {
            Operation::Assign(src) => {
                src.is_ready(&wires)
            },
            Operation::Not(op) => {
                op.is_ready(wires)
            },
            Operation::And(lhs, rhs) => {
                lhs.is_ready(wires) && rhs.is_ready(wires)
             },
            Operation::Or(lhs, rhs) => {
                lhs.is_ready(wires) && rhs.is_ready(wires)
             },
            Operation::Lshift(lhs, rhs) => {
                lhs.is_ready(wires) && rhs.is_ready(wires)
            },
            Operation::Rshift(lhs, rhs) => {
                lhs.is_ready(wires) && rhs.is_ready(wires)
             },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Command {
    op: Operation,
    destination: String,
}

impl Command {
    fn execute(&self, wires: &mut Wires) {
        let result = self.op.execute(wires);
        wires.assign(&self.destination, result);
    }
    
    fn is_ready(&self, wires: &Wires) -> bool {
        self.op.is_ready(wires)
    }
}

struct Wires {
    m: HashMap<String, u16>,
}

impl Wires {
    fn new() -> Wires {
        Wires {
            m: HashMap::new(),
        }
    }

    fn wire_has_value(&self, wire_name: &str) -> bool {
        self.m.contains_key(wire_name)
    }

    fn assign(&mut self, wire_name: &str, value: u16) {
        assert!(!self.wire_has_value(wire_name));
        self.m.insert(String::from(wire_name), value);
    }

    fn value(&self, wire_name: &str) -> u16 {
        assert!(self.wire_has_value(wire_name));
        *self.m.get(wire_name).unwrap()
    }
}

fn execute_program(commands: &Vec<Command>) -> Wires {
    let mut wires = Wires::new();
    let mut skip_set = std::collections::HashSet::<usize>::new();
    while skip_set.len() != commands.len() {
        let mut did_progress = false;
        for (i, c) in commands.iter().enumerate() {
            if !skip_set.contains(&i) {
                if c.is_ready(&wires) {
                    c.execute(&mut wires);
                    skip_set.insert(i);
                    did_progress = true;
                }
            }
        }
        assert!(did_progress);
    }
    wires
}

fn parse_input(input: &str) -> Vec<Command> {
    let mut commands = Vec::<_>::new();
    let rx_immediate_number = regex::Regex::new(r"^(\d+) -> (\w+)$").expect("Invalid regex: immediate number");
    let rx_immediate_register = regex::Regex::new(r"^(\w+) -> (\w+)$").expect("Invalid regex: immediate register");
    let rx_op_not = regex::Regex::new(r"^NOT ([\w\d]+) -> (\w+)$").expect("Invalid regex: op not");
    let rx_op_binary = regex::Regex::new(r"^([\w\d]+) (AND|OR|LSHIFT|RSHIFT) ([\w\d]+) -> (\w+)$").expect("Invalid regex: op binary");
    for l in input.lines() {
         if let Some(m_immediate_number) = rx_immediate_number.captures(l) {
            let (_, [n, dest]) = m_immediate_number.extract();
            let c = Command{
                op: Operation::Assign(Operand::Number(n.parse().unwrap())),
                destination: String::from(dest),
             };
             commands.push(c);
        } else if let Some(m_immediate_register) = rx_immediate_register.captures(l) {
            let (_, [src, dest]) = m_immediate_register.extract();
            let c = Command {
                op: Operation::Assign(Operand::Register(String::from(src))),
                destination: String::from(dest),
            };
            commands.push(c);
        } else if let Some(m_op_not) = rx_op_not.captures(l) {
            let (_, [src, dest]) = m_op_not.extract();
            let c = Command {
                op: Operation::Not(Operand::parse(src)),
                destination: String::from(dest),
            };
            commands.push(c);
        } else if let Some(m_op_binary) = rx_op_binary.captures(l) {
            let (_, [op1, opcode, op2, dest]) = m_op_binary.extract();
            let operand_lhs = Operand::parse(op1);
            let operand_rhs = Operand::parse(op2);
            let c = Command {
                op: match opcode {
                    "AND" => Operation::And(operand_lhs, operand_rhs),
                    "OR" => Operation::Or(operand_lhs, operand_rhs),
                    "LSHIFT" => Operation::Lshift(operand_lhs, operand_rhs),
                    "RSHIFT" => Operation::Rshift(operand_lhs, operand_rhs),
                    _ => panic!("Invalid opcode on line: {}", l),
                },
                destination: String::from(dest),
            };
            commands.push(c);
        } else {
            panic!("Unmatched line: {}", l)
        }
    }
    commands
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn get_sample_input() -> &'static str {
        r"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"
    }

    #[test]
    fn test_parse_input() {
        let p = parse_input(get_sample_input());
        assert_eq!(p.len(), 8);
        assert_eq!(p[0], Command{ op: Operation::Assign(Operand::Number(123)), destination: String::from("x") });
        assert_eq!(p[1], Command{ op: Operation::Assign(Operand::Number(456)), destination: String::from("y") });
        assert_eq!(p[2], Command{ op: Operation::And(Operand::Register(String::from("x")), Operand::Register(String::from("y"))), destination: String::from("d") });
        assert_eq!(p[3], Command{ op: Operation::Or(Operand::Register(String::from("x")), Operand::Register(String::from("y"))), destination: String::from("e") });
        assert_eq!(p[4], Command{ op: Operation::Lshift(Operand::Register(String::from("x")), Operand::Number(2)), destination: String::from("f") });
        assert_eq!(p[5], Command{ op: Operation::Rshift(Operand::Register(String::from("y")), Operand::Number(2)), destination: String::from("g") });
        assert_eq!(p[6], Command{ op: Operation::Not(Operand::Register(String::from("x"))), destination: String::from("h") });
        assert_eq!(p[7], Command{ op: Operation::Not(Operand::Register(String::from("y"))), destination: String::from("i") });
    }

    #[test]
    fn test_execute_program() {
        let p = parse_input(get_sample_input());
        let wires = execute_program(p);
        assert_eq!(wires.m.len(), 8);
        assert_eq!(wires.value("d"), 72);
        assert_eq!(wires.value("e"), 507);
        assert_eq!(wires.value("f"), 492);
        assert_eq!(wires.value("g"), 114);
        assert_eq!(wires.value("h"), 65412);
        assert_eq!(wires.value("i"), 65079);
        assert_eq!(wires.value("x"), 123);
        assert_eq!(wires.value("y"), 456);
    }
}
