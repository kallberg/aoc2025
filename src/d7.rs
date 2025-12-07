use std::collections::HashMap;

type Row = Vec<char>;
type Grid = Vec<Row>;

fn parse_grid(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn extend_beam(row: &mut Row, index: usize) -> usize {
    let mut splits = 0;
    match row[index] {
        'S' => unreachable!(),
        '^' => {
            splits += 1;
            if index > 0 && row[index - 1] == '.' {
                row[index - 1] = '|';
            }
            if index + 1 < row.len() && row[index + 1] == '.' {
                row[index + 1] = '|';
            }
        }
        _ => row[index] = '|',
    }
    splits
}

fn eval_step(current: &Row, next: &mut Row) -> usize {
    let mut splits = 0;
    for (x, state) in current.iter().enumerate() {
        if *state == 'S' || *state == '|' {
            splits += extend_beam(next, x);
        }
    }
    splits
}

fn eval_grid(grid: &mut Grid) -> usize {
    let mut splits = 0;
    for index in 0..(grid.len() - 1) {
        let current = grid[index].clone();
        let next = &mut grid[index + 1];
        splits += eval_step(&current, next);
    }
    splits
}

fn paths_from(
    grid: &Grid,
    y: usize,
    x: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(cached) = cache.get(&(y, x)) {
        return *cached;
    }

    if y + 1 == grid.len() {
        return 1;
    }

    let next = grid[y][x];

    match next {
        'S' | '.' => paths_from(grid, y + 1, x, cache),
        '^' => {
            let mut sum = 0;
            if x > 0 {
                sum += paths_from(grid, y + 1, x - 1, cache);
            }
            if x + 1 < grid.len() {
                sum += paths_from(grid, y + 1, x + 1, cache);
            }
            cache.insert((y, x), sum);
            sum
        }
        _ => 0,
    }
}

fn walk_grid(grid: &Grid) -> usize {
    if grid.is_empty() {
        return 0;
    }
    let first_row = &grid[0];

    let Some((start, _)) = first_row.iter().enumerate().find(|(_, char)| **char == 'S') else {
        return 0;
    };

    paths_from(grid, 0, start, &mut HashMap::new())
}

pub fn part_1(input: &str) -> String {
    let mut grid = parse_grid(input);
    let splits = eval_grid(&mut grid);
    splits.to_string()
}

pub fn part_2(input: &str) -> String {
    let grid = parse_grid(input);
    walk_grid(&grid).to_string()
}
