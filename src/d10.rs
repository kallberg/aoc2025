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
}

struct Matrix {
    values: Vec<f64>,
    width: usize,
}

impl Matrix {
    fn height(&self) -> usize {
        self.values.len() / self.width
    }

    fn swap_rows(&mut self, row_a: usize, row_b: usize) {
        if row_a == row_b {
            return;
        }
        for column in 0..self.width {
            let idx_a = row_a * self.width + column;
            let idx_b = row_b * self.width + column;
            self.values.swap(idx_a, idx_b);
        }
    }

    fn row_echelon_form(&mut self) {
        let height = self.height();
        let width = self.width;
        let mut pivot_row = 0;
        let mut pivot_column = 0;

        while pivot_row < height && pivot_column < width {
            let mut max_row = pivot_row;
            for i in pivot_row + 1..height {
                if self.values[i * width + pivot_column].abs()
                    > self.values[max_row * width + pivot_column].abs()
                {
                    max_row = i;
                }
            }

            if self.values[max_row * width + pivot_column].abs() < 1e-9 {
                pivot_column += 1;
                continue;
            }

            self.swap_rows(pivot_row, max_row);

            for i in pivot_row + 1..height {
                let target_idx = i * width + pivot_column;
                let pivot_idx = pivot_row * width + pivot_column;

                let factor = self.values[target_idx] / self.values[pivot_idx];

                for j in pivot_column..width {
                    let value_to_subtract = self.values[pivot_row * width + j] * factor;
                    self.values[i * width + j] -= value_to_subtract;
                }
            }

            pivot_row += 1;
            pivot_column += 1;
        }
    }

    fn reduced_row_echelon_form(&mut self) {
        self.row_echelon_form();
        let height = self.height();
        let width = self.width;
        for row in (0..height).rev() {
            let mut pivot_column = None;
            for column in 0..width - 1 {
                if self.values[row * width + column].abs() > 1e-9 {
                    pivot_column = Some(column);
                    break;
                }
            }

            if let Some(pivot_column) = pivot_column {
                let pivot_value = self.values[row * width + pivot_column];
                for column in pivot_column..width {
                    self.values[row * width + column] /= pivot_value;
                }
                for row_above in 0..row {
                    let factor = self.values[row_above * width + pivot_column];
                    for col in pivot_column..width {
                        let val_to_subtract = self.values[row * width + col] * factor;
                        self.values[row_above * width + col] -= val_to_subtract;
                    }
                }
            }
        }
    }

    fn pivot_points(&self) -> Vec<(usize, usize)> {
        let variable_count = self.width - 1;
        let mut points = vec![];
        for row in 0..self.height() {
            for column in 0..variable_count {
                if self.values[row * self.width + column] == 1.0 {
                    points.push((row, column));
                    break;
                }
            }
        }
        points
    }

    fn free_indices_from_pivot_indices(&self, pivot_indices: &[usize]) -> Vec<usize> {
        let variable_count = self.width - 1;
        let free_indices: Vec<usize> = (0..variable_count)
            .filter(|c| !pivot_indices.contains(c))
            .collect();
        free_indices
    }

    fn free_indices(&self) -> Vec<usize> {
        let pivot_indices: Vec<usize> = self.pivot_points().iter().map(|(_, col)| *col).collect();
        self.free_indices_from_pivot_indices(&pivot_indices)
    }

    fn solve(&self, free_variables: &[f64]) -> Option<Vec<f64>> {
        let variable_count = self.width - 1;
        let mut solution: Vec<f64> = vec![0.0; variable_count];

        let pivot_points = self.pivot_points();
        let pivot_indices: Vec<usize> = pivot_points.iter().map(|(_, c)| *c).collect();
        let free_indices: Vec<usize> = self.free_indices_from_pivot_indices(&pivot_indices);

        if free_variables.len() != free_indices.len() {
            return None;
        }

        for (free_variable, column) in free_indices.iter().enumerate() {
            solution[*column] = free_variables[free_variable];
        }

        for (row, pivot_column) in pivot_points {
            let rhs = self.values[row * self.width + variable_count];
            let mut sum_free_varibles = 0.0;

            for variable_index in &free_indices {
                let variable_value = solution[*variable_index];
                let variable_constant_multiplier = self.values[row * self.width + variable_index];
                let value = variable_value * variable_constant_multiplier;
                sum_free_varibles += value;
            }

            solution[pivot_column] = rhs - sum_free_varibles;
        }

        Some(solution)
    }

    fn augmented_column(&self) -> Vec<f64> {
        (0..self.height())
            .map(|row| self.values[row * self.width + self.width - 1])
            .collect()
    }
}

fn joltage_matrix(machine: &Machine) -> Matrix {
    let width = machine.buttons.len() + 1;
    let values = {
        let mut values = vec![];
        for (index, joltage) in machine.joltage_requirements.iter().enumerate() {
            for button in &machine.buttons {
                let joltages = button_joltages(*button);
                match joltages.contains(&index) {
                    true => values.push(1.0),
                    false => values.push(0.0),
                }
            }
            values.push(*joltage as f64);
        }
        values
    };
    Matrix { values, width }
}

fn is_positive_integer_or_zero(value: f64, epsilon: f64) -> bool {
    value.round() >= 0.0 && (value - value.round()).abs() < epsilon
}

fn valid_joltage_solution(solution: &[f64]) -> bool {
    let epsilon = 1e-9;
    for value in solution {
        if !is_positive_integer_or_zero(*value, epsilon) {
            return false;
        }
    }
    true
}

fn joltage_depth_first_search(
    matrix: &Matrix,
    variable_index: usize,
    free_varibles: usize,
    values: &mut [usize],
    best_values: &mut [usize],
    best: &mut usize,
    maximum: usize,
) {
    let matrix_values: Vec<f64> = values.iter().map(|value| *value as f64).collect();
    let Some(solution) = matrix.solve(&matrix_values) else {
        return;
    };
    let current_sum = (solution.iter().sum::<f64>()).round() as usize;
    if variable_index == free_varibles {
        if valid_joltage_solution(&solution) && current_sum < *best {
            values
                .iter()
                .enumerate()
                .for_each(|(index, value)| best_values[index] = *value);
            *best = current_sum;
        }
        return;
    }

    let running_value_sum: usize = values[..variable_index].iter().sum();

    for presses in 0..=maximum {
        if running_value_sum + presses > *best {
            return;
        }
        values[variable_index] = presses;
        joltage_depth_first_search(
            matrix,
            variable_index + 1,
            free_varibles,
            values,
            best_values,
            best,
            maximum,
        );
    }
}

fn joltage_solve(machine: &Machine) -> (Vec<f64>, usize) {
    let mut matrix = joltage_matrix(machine);
    let maximum_value: usize = matrix.augmented_column().iter().sum::<f64>().round() as usize;
    matrix.reduced_row_echelon_form();
    let variable_count = matrix.free_indices().len();

    if variable_count == 0 {
        let solution = matrix.augmented_column();
        let sum: usize = solution.iter().map(|v| v.round() as usize).sum();
        return (solution, sum);
    }

    let mut variables = vec![0; matrix.free_indices().len()];
    let mut best_variables = variables.clone();
    let mut best_sum = usize::MAX;
    joltage_depth_first_search(
        &matrix,
        0,
        variable_count,
        &mut variables,
        &mut best_variables,
        &mut best_sum,
        maximum_value,
    );
    let matrix_variables: Vec<f64> = best_variables.iter().map(|v| *v as f64).collect();
    let solution = matrix.solve(&matrix_variables).unwrap();
    (solution, best_sum)
}

pub fn part_1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let machine = Machine::from(line);
        sum += machine.solve_lights(1000);
    }
    sum.to_string()
}

pub fn part_2(input: &str) -> String {
    let mut sum = 0;
    for (index, line) in input.lines().enumerate() {
        let machine = Machine::from(line);
        let (best_solution, best_sum) = joltage_solve(&machine);
        println!("Solving machine {index}, best_sum={best_sum}");
        sum += best_sum;
    }
    sum.to_string()
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
}
