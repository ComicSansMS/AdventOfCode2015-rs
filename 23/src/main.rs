fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input");
    let mut program = parse_input(&input);
    let (_a, b) = execute_program(&program);
    println!("Answer #1 is {}", b);
    program.insert(0, Instruction::Increment('a'));
    let (_a, b) = execute_program(&program);
    println!("Answer #2 is {}", b);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Half(char),
    Triple(char),
    Increment(char),
    Jump(i32),
    JumpIfEven(char, i32),
    JumpIfOne(char, i32),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut ret = Vec::new();
    for l in input.lines() {
        let opcode = &l[0..3];
        assert_eq!(l.chars().nth(3).unwrap(), ' ');
        let parse_register = |l: &str| -> char {
            let r = l.chars().nth(4).unwrap();
            assert!(r == 'a' || r == 'b');
            r
        };
        let parse_offset = |s: &str| -> i32 {
            let is_negative = match s.chars().nth(0).unwrap() {
                '+' => false,
                '-' => true,
                x => { panic!("Invalid sign {}", x); }
            };
            let value = s[1..].parse::<i32>().expect(format!("Invalid offset {}", s).as_str());
            value * (if is_negative { -1 } else { 1 })
        };
        match opcode {
            "hlf" => {
                ret.push(Instruction::Half(parse_register(l)));
            },
            "tpl" => {
                ret.push(Instruction::Triple(parse_register(l)));
            },
            "inc" => {
                ret.push(Instruction::Increment(parse_register(l)));
            },
            "jmp" => {
                ret.push(Instruction::Jump(parse_offset(&l[4..])));
            },
            "jie" => {
                assert_eq!(l.chars().nth(5).unwrap(), ',');
                assert_eq!(l.chars().nth(6).unwrap(), ' ');
                ret.push(Instruction::JumpIfEven(parse_register(l), parse_offset(&l[7..])));
            },
            "jio" => {
                assert_eq!(l.chars().nth(5).unwrap(), ',');
                assert_eq!(l.chars().nth(6).unwrap(), ' ');
                ret.push(Instruction::JumpIfOne(parse_register(l), parse_offset(&l[7..])));
            },
            _ => { panic!("Invalid opcode: {}", opcode); },
        }
    }
    ret
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Machine {
    a: usize,
    b: usize,
    ip: usize,
}

impl Machine {
    fn new() -> Self {
        Self { a: 0, b: 0, ip: 0 }
    }

    fn register(&self, r: char) -> usize {
        match r {
            'a' => self.a,
            'b' => self.b,
            _ => panic!("Invalid register"),
        }
    }

    fn register_mut(&mut self, r: char) -> &mut usize {
        match r {
            'a' => &mut self.a,
            'b' => &mut self.b,
            _ => panic!("Invalid register"),
        }
    }

    fn apply_offset(&mut self, offset: i32) {
        if offset < 0 {
            self.ip -= (offset * -1) as usize;
        } else {
            self.ip += offset as usize;
        }
    }

    fn step(&mut self, program: &[Instruction]) -> bool {
        if self.ip < program.len() {
            match program[self.ip] {
                Instruction::Half(r) => {
                    *self.register_mut(r) /= 2;
                    self.ip += 1;
                },
                Instruction::Triple(r) => {
                    *self.register_mut(r) *= 3;
                    self.ip += 1;
                },
                Instruction::Increment(r) => {
                    *self.register_mut(r) += 1;
                    self.ip += 1;
                },
                Instruction::Jump(offset) => {
                    self.apply_offset(offset);
                },
                Instruction::JumpIfEven(r, offset) => {
                    if self.register(r) % 2 == 0 {
                        self.apply_offset(offset)
                    } else {
                        self.ip += 1;
                    }
                },
                Instruction::JumpIfOne(r, offset) => {
                    if self.register(r) == 1 {
                        self.apply_offset(offset);
                    } else {
                        self.ip += 1;
                    }
                },
            }
            true
        } else {
            false
        }
    }
}

fn execute_program(program: &[Instruction]) -> (usize, usize) {
    let mut vm = Machine::new();
    loop {
        if !vm.step(program) { break; }
    }
    (vm.a, vm.b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let p = parse_input(r"inc a
jio a, +2
tpl a
inc a
hlf b
jie a, -32
jmp -5");
        assert_eq!(p.len(), 7);
        assert_eq!(p[0], Instruction::Increment('a'));
        assert_eq!(p[1], Instruction::JumpIfOne('a', 2));
        assert_eq!(p[2], Instruction::Triple('a'));
        assert_eq!(p[3], Instruction::Increment('a'));
        assert_eq!(p[4], Instruction::Half('b'));
        assert_eq!(p[5], Instruction::JumpIfEven('a', -32));
        assert_eq!(p[6], Instruction::Jump(-5));
    }

    #[test]
    fn test_execute_program() {
        let p = parse_input(r"inc a
jio a, +2
tpl a
inc a
hlf a
jie a, +2
inc b
inc b
jmp +2
inc a
tpl b");
        assert_eq!(execute_program(&p), (1, 6));
    }
}
