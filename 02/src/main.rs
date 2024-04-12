fn main() {
    let input_filename = "input";
    let input = std::fs::read_to_string(input_filename).expect("Error reading input file");

    let boxes = parse_input(&input);
    let answer1 = boxes.iter()
                            .map(|b| b.wrapping_paper_area())
                            .fold(0, |acc, x| acc + x);
    println!("Answer #1 is {}", answer1);

    let answer2 = boxes.iter()
                                            .map(|b| b.ribbon_length())
                                            .fold(0, |acc, x| acc + x );
    println!("Answer #2 is {}", answer2);
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Box {
    length: i64,
    width: i64,
    height: i64
}

impl Box {
    fn surface_area(&self) -> i64 {
        2*self.length*self.width + 2*self.width*self.height + 2*self.height*self.length
    }

    fn smallest_side_area(&self) -> i64 {
        (self.length*self.width).min(self.width*self.height).min(self.height*self.length)
    }

    fn wrapping_paper_area(&self) -> i64 {
        self.surface_area() + self.smallest_side_area()
    }

    fn volume(&self) -> i64 {
        self.length * self.width * self.height
    }

    fn smallest_circumference(&self) -> i64 {
        let mut v = vec![self.length, self.width, self.height];
        v.sort();
        2* (v[0] + v[1])
    }

    fn ribbon_length(&self) -> i64 {
        self.smallest_circumference() + self.volume()
    }
}

fn parse_input(input: &str) -> Vec<Box> {
    let mut boxes = Vec::<Box>::new();
    for l in input.lines() {
        let b = parse_line(l);
        boxes.push(b);
    }
    boxes
}

fn parse_line(line: &str) -> Box {
    let triple: Vec<_> = line.split('x').collect();
    assert_eq!(triple.len(), 3);
    Box {
        length: triple[0].parse().unwrap(),
        width: triple[1].parse().unwrap(),
        height: triple[2].parse().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::parse_line;
    use crate::Box;

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_line("1x2x3"), Box{ length: 1, width: 2, height: 3 });
        assert_eq!(parse_line("55x222x333"), Box{ length: 55, width: 222, height: 333 });
    }

    #[test]
    fn test_surface_area() {
        assert_eq!(Box{ length: 2, width: 3, height: 4 }.surface_area(), 52);
        assert_eq!(Box{ length: 1, width: 1, height: 10 }.surface_area(), 42);
    }

    #[test]
    fn test_smallest_side_area() {
        assert_eq!(Box{ length: 2, width: 3, height: 4 }.smallest_side_area(), 6);
        assert_eq!(Box{ length: 1, width: 1, height: 10 }.smallest_side_area(), 1);
    }

    #[test]
    fn test_wrapping_paper_area() {
        assert_eq!(Box{ length: 2, width: 3, height: 4 }.wrapping_paper_area(), 58);
        assert_eq!(Box{ length: 1, width: 1, height: 10 }.wrapping_paper_area(), 43);
    }
    
    #[test]
    fn test_volume() {
        assert_eq!(Box{ length: 2, width: 3, height: 4 }.volume(), 24);
        assert_eq!(Box{ length: 1, width: 1, height: 10 }.volume(), 10);
    }
    
    #[test]
    fn test_smallest_circumference() {
        assert_eq!(Box{ length: 2, width: 3, height: 4 }.smallest_circumference(), 10);
        assert_eq!(Box{ length: 1, width: 1, height: 10 }.smallest_circumference(), 4);
    }

    #[test]
    fn test_ribbon_length() {
        assert_eq!(Box{ length: 2, width: 3, height: 4 }.ribbon_length(), 34);
        assert_eq!(Box{ length: 1, width: 1, height: 10 }.ribbon_length(), 14);
    }
}
