use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    mem::swap,
};

type Point = (u8, u8);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Present {
    width: u8,
    height: u8,
    parts: Vec<Point>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Area {
    width: u8,
    height: u8,
    filled: Vec<Point>,
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
            filled: vec![],
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

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            filled: vec![],
        }
    }

    fn place(&mut self, offset_x: u8, offset_y: u8, present: &Present) -> bool {
        if offset_x + present.width > self.width || offset_y + present.height > self.height {
            return false;
        }
        // // Assumption: We only need to test spots that have any filled neighbour
        // // Assumption: Neighbours on diagonals do not count
        // // Caveat: Except when area is empty
        // // Disabled because it never happens somehow
        // if !self.filled.is_empty() {
        //     let left = offset_x > 0 && self.filled.contains(&(offset_x - 1, offset_y));
        //     let right = self.filled.contains(&(offset_x + 1, offset_y));
        //     let top = offset_y > 0 && self.filled.contains(&(offset_x, offset_y - 1));
        //     let bottom = self.filled.contains(&(offset_x, offset_y + 1));

        //     if !left && !right && !top && !bottom {
        //         println!("!");
        //         return false;
        //     }
        // }
        let mut next = self.filled.clone();
        for (part_x, part_y) in &present.parts {
            let x = part_x + offset_x;
            let y = part_y + offset_y;
            if next.contains(&(x, y)) {
                return false;
            }
            next.push((x, y));
        }
        self.filled = next;
        true
    }

    fn try_place(&mut self, present: &Present) -> bool {
        // TODO: Add outer loop with each variant
        for offset_y in 0..self.width {
            for offset_x in 0..self.height {
                if self.place(offset_x, offset_y, present) {
                    return true;
                }
            }
        }
        false
    }

    fn try_place_variants(&self, variants: &[Present]) -> Vec<Area> {
        let mut output = vec![];
        for variant in variants {
            let mut next = self.clone();
            if next.try_place(variant) && !output.contains(&next) {
                output.push(next);
            }
        }
        output
    }

    fn score(&self) -> usize {
        let mut mx = 0;
        let mut my = 0;
        for (x, y) in self.filled.iter() {
            mx = mx.max(*x as usize + 1);
            my = my.max(*y as usize + 1);
        }
        mx * my - self.filled.len()
    }

    fn fit(
        &self,
        presents: &[Present],
        variants_map: &HashMap<Present, Vec<Present>>,
        cache: &mut HashSet<(Area, Vec<Present>)>,
        best: &mut usize,
    ) -> Option<Area> {
        if cache.contains(&(self.clone(), presents.to_vec())) {
            return None;
        }

        if presents.is_empty() {
            return Some(self.clone());
        }

        let present = &presents[0];
        let variants = variants_map.get(present).unwrap();
        let mut nexts = self.try_place_variants(variants);

        if nexts.is_empty() {
            cache.insert((self.clone(), presents.to_vec()));
            return None;
        }

        nexts.sort_unstable_by_key(|a| a.score());

        let previous_best = *best;

        if presents.len() < *best {
            println!("{self}");
            *best = presents.len();
        }

        if presents.len() == 1 && !nexts.is_empty() {
            return Some(nexts[0].clone());
        }
        let rest = &presents[1..];

        for next in nexts {
            for permutation in rest.iter().cloned().permutations(rest.len()) {
                if let Some(area) = next.fit(&permutation, variants_map, cache, best) {
                    return Some(area);
                };
            }
        }

        *best = previous_best;

        cache.insert((self.clone(), presents.to_vec()));
        None
    }
}

fn parse_input(input: &str) -> (Vec<Present>, Vec<(Area, Vec<usize>)>) {
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
    let mut area_and_requirements = vec![];

    for puzzle_str in puzzles_str.lines() {
        let (area_str, requirement_str) = puzzle_str.split_once(": ").unwrap();
        let requirements: Vec<usize> = requirement_str
            .split_whitespace()
            .map(|item| item.parse().unwrap())
            .collect();
        area_and_requirements.push((Area::from(area_str), requirements));
    }

    (presents, area_and_requirements)
}

fn required_presents(types: &[Present], requirements: Vec<usize>) -> Vec<Present> {
    requirements
        .into_iter()
        .enumerate()
        .filter(|(_index, count)| *count > 0)
        .flat_map(|(index, count)| vec![types[index].clone(); count])
        .collect()
}

pub fn part_1(input: &str) -> String {
    // let (presents, area_requirements) = parse_input(input);
    // let mut sum = 0;

    // let mut variants_map = HashMap::new();

    // for present in &presents {
    //     variants_map.insert(present.clone(), present.variants());
    // }

    // for (index, (area, requirements)) in area_requirements.into_iter().enumerate() {
    //     let area_presents = required_presents(&presents, requirements);
    //     let mut best = usize::MAX;
    //     if let Some(fitted) = area.fit(
    //         &area_presents,
    //         &variants_map,
    //         &mut HashSet::new(),
    //         &mut best,
    //     ) {
    //         println!("area {index} can fit its requirements");
    //         println!("{fitted}\n");
    //         sum += 1;
    //     }
    // }

    // sum.to_string()
    let (presents, area_requirements) = parse_input(input);
    let mut sum = 0;

    for (area, requirements) in area_requirements.into_iter() {
        let area_presents = required_presents(&presents, requirements);
        if area_presents
            .into_iter()
            .map(|present| present.parts.len() as u16)
            .sum::<u16>()
            <= (area.width as u16 * area.height as u16)
        {
            sum += 1;
        }
    }

    sum.to_string()
}

pub fn part_2(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use crate::{
        d12::{Area, Present, flip, flip_x, parse_input},
        input,
    };

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
    }

    #[test]
    fn test_second_example() {
        let (presents, area_requirements) = parse_input(input::D12E);
        let (mut area, requirements) = area_requirements[1].clone();

        let mut p2 = presents[5].clone();
        p2.clockwise_rotate();
        let p3 = presents[2].clone();
        let mut p4 = presents[4].clone();
        p4.flip_x();
        let p5 = presents[4].clone();

        assert!(area.place(4, 0, &presents[0]));
        println!("{area}");
        assert!(area.place(8, 2, &p2));
        println!("{area}");
        assert!(area.place(9, 0, &p2));
        println!("{area}");
        assert!(area.place(6, 0, &p3));
        println!("{area}");
        assert!(area.place(1, 1, &p4));
        println!("{area}");

        assert!(area.try_place(&p5));
        println!("{area}");
    }
}
