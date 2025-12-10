// Assumptions:
// 1. Buttons can only be numbered from 0-9
// 2. From 1. follows that there can be light diagram of at most size 10
//

use std::collections::HashSet;

const BUTTON_MASK: [u64; 10] = [
    0b0000000001,
    0b0000000010,
    0b0000000100,
    0b0000001000,
    0b0000010000,
    0b0000100000,
    0b0001000000,
    0b0010000000,
    0b0100000000,
    0b1000000000,
];

struct Machine {
    needed_lights: u64,
    buttons: Vec<u64>,
    #[allow(dead_code)]
    joltage_requirements: Vec<u64>,
}

fn light_diagram_from_str(string: &str) -> u64 {
    let mut diagram = 0;
    for char in string.chars().rev() {
        match char {
            '.' => diagram <<= 1,
            '#' => diagram = (diagram << 1) | 1,
            _ => continue,
        }
    }
    diagram
}

fn button_from_str(string: &str) -> u64 {
    let mut button = 0;
    for char in string.chars() {
        match char {
            '0' => button |= BUTTON_MASK[0],
            '1' => button |= BUTTON_MASK[1],
            '2' => button |= BUTTON_MASK[2],
            '3' => button |= BUTTON_MASK[3],
            '4' => button |= BUTTON_MASK[4],
            '5' => button |= BUTTON_MASK[5],
            '6' => button |= BUTTON_MASK[6],
            '7' => button |= BUTTON_MASK[7],
            '8' => button |= BUTTON_MASK[8],
            '9' => button |= BUTTON_MASK[9],
            _ => continue,
        }
    }
    button
}

fn joltage_from_str(string: &str) -> Vec<u64> {
    let end = string.len() - 1;
    // Remove prefix and suffix brackets
    let inner = &string[1..end];

    inner.split(',').map(|part| part.parse().unwrap()).collect()
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() < 3 {
            panic!("need 3 parts or more to describe a machine");
        }
        let len = parts.len();
        let light_diagram_str = parts[0];
        let joltage_requirements_str = parts[len - 1];
        let buttons_str = &parts[1..(len - 1)];

        let needed = light_diagram_from_str(light_diagram_str);
        let mut buttons = vec![];
        for button_str in buttons_str {
            buttons.push(button_from_str(button_str));
        }
        let joltage_requirements = joltage_from_str(joltage_requirements_str);

        Self {
            needed_lights: needed,
            buttons,
            joltage_requirements,
        }
    }
}

fn light_alternatives(current: u64, buttons: &[u64]) -> Vec<u64> {
    let mut output = vec![];
    for button in buttons {
        output.push(current ^ button);
    }
    output
}

fn button_joltages(button: u64) -> Vec<usize> {
    (0..10)
        .filter(|index| button & BUTTON_MASK[*index] > 0)
        .collect()
}

fn required_buttons(buttons: &[u64], requirement: &[u64]) -> Vec<(u64, usize)> {
    for (index, count) in requirement.iter().enumerate() {
        let mut providing_buttons = vec![];
        for button in buttons {
            for joltage in button_joltages(button) {}
        }
    }
}

fn joltage_alternatives(current: &[u64], buttons: &[u64], target: &[u64]) -> Vec<Vec<u64>> {
    let mut output = vec![];

    for button in buttons {
        let mut alternative: Vec<u64> = current.to_vec();
        for index in button_joltages(*button) {
            alternative[index] += 1;
            if alternative[index] > target[index] {
                // This button press would overshoot the target - SKIP
                continue;
            }
        }
        output.push(alternative);
    }

    output
}

impl Machine {
    fn solve_lights(&self, max_depth: usize) -> usize {
        let mut iteration = 0;
        // Record of seen states to avoid loops
        let mut seen_states: HashSet<u64> = HashSet::new();
        seen_states.insert(0);
        let mut search_space = vec![0];

        loop {
            if iteration >= max_depth {
                panic!("failed to solve machine within {max_depth} iterations");
            }

            let mut next_state = vec![];

            for light in &search_space {
                for alternative in light_alternatives(*light, &self.buttons) {
                    if alternative == self.needed_lights {
                        return iteration + 1;
                    }
                    if !seen_states.contains(&alternative) {
                        seen_states.insert(alternative);
                        next_state.push(alternative);
                    }
                }
            }

            search_space = next_state;

            iteration += 1;
        }
    }

    fn solve_joltage(&self, max_depth: usize) -> usize {
        let mut iteration = 0;
        let mut search_space = vec![vec![0; self.joltage_requirements.len()]];

        loop {
            let len = search_space.len();
            println!("solve_joltage iteration={iteration} search_space_len={len}");
            if iteration >= max_depth {
                panic!("failed to solve machine within {max_depth} iterations");
            }

            let mut next_state = vec![];

            for joltage in &search_space {
                for alternative in
                    joltage_alternatives(joltage, &self.buttons, &self.joltage_requirements)
                {
                    if alternative == self.joltage_requirements {
                        return iteration + 1;
                    }
                    next_state.push(alternative);
                }
            }

            search_space = next_state;

            iteration += 1;
        }
    }
}

pub fn part_1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let machine = Machine::from(line);
        sum += machine.solve_lights(1000);
    }
    sum.to_string()
}

pub fn part_2(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use crate::d10::{BUTTON_MASK, Machine, button_joltages};

    #[test]
    fn test_expected_machine_definition() {
        let machine = Machine::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(machine.needed_lights, 0b0110);
        assert_eq!(machine.buttons.len(), 6);
        assert_eq!(machine.buttons[0], 0b1000);
        assert_eq!(machine.buttons[1], 0b1010);
        assert_eq!(machine.buttons[2], 0b0100);
        assert_eq!(machine.buttons[3], 0b1100);
        assert_eq!(machine.buttons[4], 0b0101);
        assert_eq!(machine.buttons[5], 0b0011);
        assert_eq!(machine.joltage_requirements, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_example_machine_light_solutions() {
        let m1 = Machine::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        let m2 = Machine::from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        let m3 = Machine::from("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(m1.solve_lights(2), 2);
        assert_eq!(m2.solve_lights(3), 3);
        assert_eq!(m3.solve_lights(2), 2);
    }

    #[test]
    fn test_button_joltage_conversion() {
        assert_eq!(button_joltages(BUTTON_MASK[0] | BUTTON_MASK[1]), vec![0, 1])
    }

    #[test]
    fn test_example_machine_joltage_solutions() {
        let m1 = Machine::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        let m2 = Machine::from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        let m3 = Machine::from("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(m1.solve_joltage(10), 10);
        assert_eq!(m2.solve_joltage(12), 12);
        assert_eq!(m3.solve_joltage(11), 11);
    }
}
