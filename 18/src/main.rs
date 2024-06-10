fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input");
    let f = Field::from_input(&input);
    let mut fi = f.clone();
    for _ in 0..100 { fi = fi.step(); }
    println!("Answer #1 is {}", fi.light_count());
    fi = f;
    fi.corners_on();
    for _ in 0..100 { fi = fi.step2(); }
    println!("Answer #2 is {}", fi.light_count());
}

#[derive(Clone)]
struct Field {
    cells: Vec<bool>,
    width: usize,
    height: usize,
}

impl Field {
    fn new(width: usize, height: usize) -> Self {
        let mut v = Vec::new();
        v.resize((width * height) as usize, false);
        Field { cells: v, width, height }
    }

    fn from_input(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        assert!(lines.iter().all(|v| v.len() == width));
        let mut f = Field::new(width, height);
        for (iy, &l) in lines.iter().enumerate() {
            for (ix, c) in l.chars().enumerate() {
                match c {
                    '#' => f.set_cell(ix, iy, true),
                    '.' => f.set_cell(ix, iy, false),
                    _ => panic!("Unexpected character: {}", c),
                }
            }
        }
        f
    }

    fn set_cell(&mut self, x: usize, y: usize, value: bool) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.cells[y*self.width + x] = value;
    }

    fn cell(&self, x: usize, y: usize) -> bool {
        assert!(x < self.width);
        assert!(y < self.height);
        self.cells[y*self.width + x]
    }

    fn count_neighbors(&self, x: usize, y: usize) -> i32 {
        assert!(x < self.width);
        assert!(y < self.height);
        let mut count = 0;
        // ul
        if x > 0 && y > 0 { if self.cell(x - 1, y - 1) { count += 1; } }
        // u
        if y > 0 { if self.cell(x, y - 1) { count += 1; } }
        // ur
        if x < self.width - 1 && y > 0 { if self.cell(x + 1, y - 1) { count += 1; } }
        // l
        if x > 0 { if self.cell(x - 1, y) { count += 1; } }
        // r
        if x < self.width - 1 { if self.cell(x + 1, y) { count += 1; } }
        // bl
        if x > 0 && y < self.height - 1 { if self.cell(x - 1, y + 1) { count += 1; } }
        // b
        if y < self.height - 1 { if self.cell(x, y + 1) { count += 1; } }
        // br
        if x < self.width - 1 && y < self.height - 1 { if self.cell(x + 1, y + 1) { count += 1; } }
        count
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for iy in 0..self.height {
            for ix in 0..self.width {
                f.write_char(if self.cell(ix, iy) { '#' } else { '.' })?
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Field {
    fn step(&self) -> Field {
        let mut f = Field::new(self.width, self.height);
        for iy in 0..self.height {
            for ix in 0..self.width {
                let n = self.count_neighbors(ix, iy);
                if self.cell(ix, iy) {
                    if n == 2 || n == 3 {
                        f.set_cell(ix, iy, true);
                    }
                } else {
                    if n == 3 {
                        f.set_cell(ix, iy, true);
                    }
                }
            }
        }
        f
    }

    fn light_count(&self) -> i32 {
        self.cells.iter().fold(0, |acc, &c| acc + if c { 1 } else { 0 })
    }

    fn corners_on(&mut self) {
        self.set_cell(0, 0, true);
        self.set_cell(self.width - 1, 0, true);
        self.set_cell(0, self.height - 1, true);
        self.set_cell(self.width - 1, self.height - 1, true);
    }

    fn step2(&self) -> Field {
        let mut f = self.step();
        f.corners_on();
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        r".#.#.#
...##.
#....#
..#...
#.#..#
####.."
    }

    #[test]
    fn test_parse_input() {
        let f = Field::from_input(&sample_input());
        assert_eq!(f.width, 6);
        assert_eq!(f.height, 6);
        assert_eq!(format!("{}", f), sample_input().to_owned() + "\n".into());
    }

    #[test]
    fn test_count_neighbors() {
        let f = Field::from_input(&sample_input());
        assert_eq!(f.count_neighbors(0, 0), 1);
        assert_eq!(f.count_neighbors(5, 0), 1);
        assert_eq!(f.count_neighbors(2, 5), 3);
    }

    #[test]
    fn test_step() {
        let mut f = Field::from_input(&sample_input());
        f = f.step();
        assert_eq!(format!("{f}"), concat!("..##..", "\n",
                                           "..##.#", "\n",
                                           "...##.", "\n",
                                           "......", "\n",
                                           "#.....", "\n",
                                           "#.##..", "\n"));
        f = f.step();
        assert_eq!(format!("{f}"), concat!("..###.", "\n",
                                           "......", "\n",
                                           "..###.", "\n",
                                           "......", "\n",
                                           ".#....", "\n",
                                           ".#....", "\n"));
        f = f.step();
        assert_eq!(format!("{f}"), concat!("...#..", "\n",
                                           "......", "\n",
                                           "...#..", "\n",
                                           "..##..", "\n",
                                           "......", "\n",
                                           "......", "\n"));
        f = f.step();
        assert_eq!(format!("{f}"), concat!("......", "\n",
                                           "......", "\n",
                                           "..##..", "\n",
                                           "..##..", "\n",
                                           "......", "\n",
                                           "......", "\n"));
        assert_eq!(f.light_count(), 4);
    }

    #[test]
    fn test_step2() {
        let mut f = Field::from_input(&sample_input());
        f.corners_on();
        assert_eq!(format!("{f}"), concat!("##.#.#", "\n",
                                           "...##.", "\n",
                                           "#....#", "\n",
                                           "..#...", "\n",
                                           "#.#..#", "\n",
                                           "####.#", "\n"));
        f = f.step2();
        assert_eq!(format!("{f}"), concat!("#.##.#", "\n",
                                           "####.#", "\n",
                                           "...##.", "\n",
                                           "......", "\n",
                                           "#...#.", "\n",
                                           "#.####", "\n"));
        f = f.step2();
        assert_eq!(format!("{f}"), concat!("#..#.#", "\n",
                                           "#....#", "\n",
                                           ".#.##.", "\n",
                                           "...##.", "\n",
                                           ".#..##", "\n",
                                           "##.###", "\n"));
        f = f.step2();
        assert_eq!(format!("{f}"), concat!("#...##", "\n",
                                           "####.#", "\n",
                                           "..##.#", "\n",
                                           "......", "\n",
                                           "##....", "\n",
                                           "####.#", "\n"));
        f = f.step2();
        assert_eq!(format!("{f}"), concat!("#.####", "\n",
                                           "#....#", "\n",
                                           "...#..", "\n",
                                           ".##...", "\n",
                                           "#.....", "\n",
                                           "#.#..#", "\n"));
        f = f.step2();
        assert_eq!(format!("{f}"), concat!("##.###", "\n",
                                           ".##..#", "\n",
                                           ".##...", "\n",
                                           ".##...", "\n",
                                           "#.#...", "\n",
                                           "##...#", "\n"));
        assert_eq!(f.light_count(), 17);
    }

}
