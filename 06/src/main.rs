fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input file");

    let instructions = parse_input(&input);
    let mut f = Field::new();
    for i in &instructions {
        process_instruction(&mut f, i);
    }

    println!("Answer #1 is {}", f.count());

    let mut f = Field::new();
    for i in &instructions {
        process_instruction2(&mut f, i);
    }
    println!("Answer #2 is {}", f.count());
}

struct Field {
    cells: Vec<u32>,
}

impl Field {
    fn new() -> Field {
        Field {
            cells: vec![0; 1000 * 1000],
        }
    }

    fn get(&self, x: usize, y: usize) -> u32 {
        (*self.cells)[y * 1000 + x]
    }

    fn update(&mut self, x: usize, y: usize, value: u32) {
        (*self.cells)[y * 1000 + x] = value;
    }

    fn count(&self) -> i32 {
        self.cells.iter().fold(0, |acc, &x| acc + (x as i32))
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let rx = regex::Regex::new(r"(toggle|turn off|turn on) (\d+),(\d+) through (\d+),(\d+)")
        .expect("Error parsing regex");
    let mut instructions = Vec::<Instruction>::new();
    for m in rx.captures_iter(input) {
        let (_, [c, x1, y1, x2, y2]) = m.extract();
        let cmd: Command = parse_command(c);
        let from = Point {
            x: x1.parse().unwrap(),
            y: y1.parse().unwrap(),
        };
        assert!((0..1000).contains(&from.x) && (0..1000).contains(&from.y));
        let to = Point {
            x: x2.parse().unwrap(),
            y: y2.parse().unwrap(),
        };
        assert!((0..1000).contains(&to.x) && (0..1000).contains(&to.y));
        instructions.push(Instruction {
            command: cmd,
            from,
            to,
        });
    }
    instructions
}

fn parse_command(c: &str) -> Command {
    match c {
        "turn on" => Command::TurnOn,
        "turn off" => Command::TurnOff,
        "toggle" => Command::Toggle,
        _ => panic!("Invalid command"),
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    command: Command,
    from: Point,
    to: Point,
}

fn process_instruction(f: &mut Field, instruction: &Instruction) {
    let op = match instruction.command {
        Command::TurnOn => |_c: u32| -> u32 { 1 },
        Command::TurnOff => |_c: u32| -> u32 { 0 },
        Command::Toggle => |c: u32| -> u32 { 1 - c },
    };
    for iy in instruction.from.y..(instruction.to.y + 1) {
        let y = iy as usize;
        for ix in instruction.from.x..(instruction.to.x + 1) {
            let x = ix as usize;
            f.update(x, y, op(f.get(x, y)));
        }
    }
}

fn process_instruction2(f: &mut Field, instruction: &Instruction) {
    let op = match instruction.command {
        Command::TurnOn => |c: u32| -> u32 { c + 1 },
        Command::TurnOff => |c: u32| -> u32 {
            if c == 0 {
                0
            } else {
                c - 1
            }
        },
        Command::Toggle => |c: u32| -> u32 { c + 2 },
    };
    for iy in instruction.from.y..(instruction.to.y + 1) {
        let y = iy as usize;
        for ix in instruction.from.x..(instruction.to.x + 1) {
            let x = ix as usize;
            f.update(x, y, op(f.get(x, y)));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_input() {
        let test_input = concat!(
            "turn on 0,0 through 999,999",
            "\n",
            "toggle 0,0 through 999,0",
            "\n",
            "turn off 499,499 through 500,500"
        );
        let instr = parse_input(test_input);
        assert_eq!(instr.len(), 3);
        assert_eq!(instr[0].command, Command::TurnOn);
        assert_eq!(instr[0].from, Point { x: 0, y: 0 });
        assert_eq!(instr[0].to, Point { x: 999, y: 999 });

        assert_eq!(instr[1].command, Command::Toggle);
        assert_eq!(instr[1].from, Point { x: 0, y: 0 });
        assert_eq!(instr[1].to, Point { x: 999, y: 0 });

        assert_eq!(instr[2].command, Command::TurnOff);
        assert_eq!(instr[2].from, Point { x: 499, y: 499 });
        assert_eq!(instr[2].to, Point { x: 500, y: 500 });
    }

    #[test]
    fn test_process_instruction() {
        let mut f = Field::new();
        process_instruction(
            &mut f,
            &Instruction {
                command: Command::TurnOn,
                from: Point { x: 0, y: 0 },
                to: Point { x: 999, y: 999 },
            },
        );
        for iy in 0..1000 {
            for ix in 0..1000 {
                assert_eq!(f.get(ix, iy), 1);
            }
        }
        process_instruction(
            &mut f,
            &Instruction {
                command: Command::Toggle,
                from: Point { x: 0, y: 0 },
                to: Point { x: 999, y: 0 },
            },
        );
        for iy in 0..1000 {
            for ix in 0..1000 {
                assert_eq!(f.get(ix, iy), if iy == 0 { 0 } else { 1 });
            }
        }
    }
}
