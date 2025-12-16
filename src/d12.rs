use std::{collections::HashSet, fmt::Display, mem::swap};

type Point = (u8, u8);

#[derive(Clone, PartialEq, Eq, Hash)]
struct Present {
    width: u8,
    height: u8,
    parts: Vec<Point>,
}

struct Area {
    width: u8,
    height: u8,
    filled: HashSet<Point>,
    placed: Vec<Present>,
}

impl From<&str> for Present {
    fn from(value: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut parts = vec![];
        for (y, line) in value.lines().enumerate() {
            height = height.max(y as u8 + 1);
            for (x, char) in line.chars().enumerate() {
                width = width.max(x as u8 + 1);
                match char {
                    '#' => parts.push((x as u8, y as u8)),
                    '.' => {}
                    _ => unreachable!(),
                }
            }
        }

        Self {
            width,
            height,
            parts,
        }
    }
}

impl Display for Present {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.parts.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<&str> for Area {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once("x").unwrap();
        Self {
            width: left.parse().unwrap(),
            height: right.parse().unwrap(),
            filled: HashSet::new(),
            placed: vec![],
        }
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.filled.contains(&(x, y)) {
                    true => write!(f, "#")?,
                    false => write!(f, ".")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn flip(value: u8, size: u8) -> u8 {
    size - value - 1
}

fn diag(point: Point) -> Point {
    (point.1, point.0)
}

fn flip_x(point: Point, width: u8) -> Point {
    let x = flip(point.0, width);
    let y = point.1;
    (x, y)
}

fn flip_y(point: Point, height: u8) -> Point {
    let x = point.0;
    let y = flip(point.1, height);
    (x, y)
}

impl Present {
    fn flip_x(&mut self) {
        for index in 0..self.parts.len() {
            self.parts[index] = flip_x(self.parts[index], self.width);
        }
    }

    fn flip_y(&mut self) {
        for index in 0..self.parts.len() {
            self.parts[index] = flip_y(self.parts[index], self.height);
        }
    }

    fn diag_flip(&mut self) {
        for index in 0..self.parts.len() {
            self.parts[index] = diag(self.parts[index]);
        }

        swap(&mut self.width, &mut self.height);
    }

    fn clockwise_rotate(&mut self) {
        self.diag_flip();
        self.flip_x();
    }

    fn counter_clockwise_rotate(&mut self) {
        self.diag_flip();
        self.flip_y();
    }

    fn flips(&self) -> Vec<Present> {
        let mut output = vec![];
        let mut next = self.clone();
        output.push(self.clone());
        next.flip_x();
        output.push(next.clone());
        next.flip_y();
        output.push(next.clone());
        next = self.clone();
        next.flip_y();
        output.push(next.clone());
        output
    }

    fn rotations(&self) -> Vec<Present> {
        let mut output = vec![];
        let mut next = self.clone();
        output.push(next.clone());
        next.clockwise_rotate();
        output.push(next.clone());
        next.clockwise_rotate();
        output.push(next.clone());
        next.clockwise_rotate();
        output.push(next.clone());
        output
    }

    fn variants(&self) -> Vec<Present> {
        // TODO: Make the follow smarter
        let mut items = vec![];
        items.extend(self.flips());
        items.extend(self.rotations());
        let mut set = HashSet::new();
        for entry in items {
            set.insert(entry);
        }
        set.into_iter().collect()
    }
}

impl Area {
    fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            filled: HashSet::new(),
            placed: vec![],
        }
    }

    fn place(&mut self, offset_x: u8, offset_y: u8, present: &Present) -> bool {
        if offset_x + present.width > self.width || offset_y + present.height > self.height {
            return false;
        }
        let mut next = self.filled.clone();
        for (part_x, part_y) in &present.parts {
            let x = part_x + offset_x;
            let y = part_y + offset_y;
            if !next.insert((x, y)) {
                return false;
            }
        }
        self.filled = next;
        true
    }

    fn try_place(&mut self, present: &Present) -> bool {
        // TODO: Add outer loop with each variant
        for offset_y in 0..self.width {
            for offset_x in 0..self.height {
                for variant in present.variants() {
                    if self.place(offset_x, offset_y, &variant) {
                        self.placed.push(variant);
                        return true;
                    }
                }
            }
        }
        false
    }
}

pub fn part_1(input: &str) -> String {
    let parts = input.split("\n\n");
    let present_strs: Vec<&str> = parts
        .clone()
        .flat_map(|lines| {
            let (_prefix, lines) = lines.split_once(":\n")?;
            Some(lines)
        })
        .collect();

    let puzzles_str = parts.last().unwrap();

    let presents: Vec<Present> = present_strs.into_iter().map(Present::from).collect();
    let mut areas = vec![];

    for puzzle_str in puzzles_str.lines() {
        let (area_str, requirement_str) = puzzle_str.split_once(" ").unwrap();
        areas.push(Area::from(area_str));
    }

    String::new()
}

pub fn part_2(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use crate::d12::{Area, Present, flip, flip_x};

    #[test]
    fn test_flip_twice_is_id() {
        for value in 1..u8::MAX {
            for size in (value + 1)..=u8::MAX {
                let flipped = flip(value, size);
                let id = flip(flipped, size);
                assert_eq!(id, value);
            }
        }
    }

    #[test]
    fn test_flip_point_x() {
        assert_eq!(flip_x((0, 0), 3), (2, 0));
    }

    #[test]
    fn test_parse_present() {
        let input_str = "###\n#..\n";
        let p = Present::from(input_str);
        let p_str = format!("{p}");
        assert_eq!(p_str, input_str);
    }

    #[test]
    fn test_flip_x() {
        let input_str = "###\n#..\n";
        let flip_str = "###\n..#\n";
        let mut p = Present::from(input_str);
        p.flip_x();
        let p_str = format!("{p}");
        assert_eq!(p_str, flip_str);
    }

    #[test]
    fn test_flip_y() {
        let input_str = "###\n#..\n";
        let flip_str = "#..\n###\n";
        let mut p = Present::from(input_str);
        p.flip_y();
        let p_str = format!("{p}");
        assert_eq!(p_str, flip_str);
    }

    #[test]
    fn test_diag_flip_id() {
        let input_str = "###\n#..\n";
        let flip_str = "###\n#..\n";
        let mut p = Present::from(input_str);
        p.diag_flip();
        p.diag_flip();
        let p_str = format!("{p}");
        assert_eq!(p_str, flip_str);
    }

    #[test]
    fn test_cw_rotate() {
        let input_str = "###\n#..\n";
        let flip_str = "##\n.#\n.#\n";
        let mut p = Present::from(input_str);
        p.clockwise_rotate();
        let p_str = format!("{p}");
        assert_eq!(p_str, flip_str);
    }

    #[test]
    fn test_cw_rotate_id() {
        let input_str = "###\n#..\n";
        let mut p = Present::from(input_str);
        p.clockwise_rotate();
        p.clockwise_rotate();
        p.clockwise_rotate();
        p.clockwise_rotate();
        let p_str = format!("{p}");
        assert_eq!(p_str, input_str);
    }

    #[test]
    fn test_ccw_rotate() {
        let input_str = "###\n#..\n";
        let flip_str = "#.\n#.\n##\n";
        let mut p = Present::from(input_str);
        p.counter_clockwise_rotate();
        let p_str = format!("{p}");
        assert_eq!(p_str, flip_str);
    }

    #[test]
    fn test_place() {
        let input_str = "###\n#..\n";
        let p = Present::from(input_str);
        let mut area = Area::new(5, 5);
        let placed = area.place(1, 1, &p);
        assert!(placed)
    }

    #[test]
    fn test_invalid_place() {
        let input_str = "###\n#..\n";
        let p = Present::from(input_str);
        let mut area = Area::new(5, 5);
        let placed = area.place(3, 1, &p);
        assert!(!placed)
    }

    #[test]
    fn test_try_place() {
        let input_str = "###\n#..\n";
        let p = Present::from(input_str);
        let mut area = Area::new(4, 10);
        {
            let placed = area.try_place(&p);
            assert!(placed);
        }
        let placed = area.try_place(&p);
        assert!(placed);
        println!("{area}");
    }
}
