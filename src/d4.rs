type Grid = Vec<Vec<bool>>;

fn parse_roll(input: u8) -> bool {
    input == b'@'
}

fn parse_grid(input: &str) -> Grid {
    let mut grid = vec![];
    for line_str in input.lines() {
        let mut row = vec![];
        for byte in line_str.bytes() {
            row.push(parse_roll(byte));
        }
        grid.push(row);
    }
    grid
}

fn roll_at(x: usize, y: usize, grid: &Grid) -> bool {
    let Some(row) = grid.get(y) else {
        return false;
    };
    let Some(cell) = row.get(x) else {
        return false;
    };
    *cell
}

fn adjacent(x: usize, y: usize) -> Vec<(usize, usize)> {
    let range_x = x.saturating_sub(1)..=(x + 1);
    let range_y = y.saturating_sub(1)..=(y + 1);
    let mut values = vec![];

    for r_y in range_y {
        for r_x in range_x.clone() {
            if r_y == y && r_x == x {
                continue;
            }
            values.push((r_x, r_y));
        }
    }

    values
}

fn adjacent_rolls(x: usize, y: usize, grid: &Grid) -> usize {
    let mut counter = 0;
    for (x, y) in adjacent(x, y) {
        if roll_at(x, y, grid) {
            counter += 1;
        }
    }
    counter
}

fn removable(x: usize, y: usize, grid: &Grid) -> bool {
    adjacent_rolls(x, y, grid) < 4
}

fn roll_char(roll: bool) -> char {
    match roll {
        true => '@',
        false => '.',
    }
}

fn row_str(row: &Vec<bool>) -> String {
    let mut string = String::new();
    for value in row {
        string.push(roll_char(*value));
    }
    string
}

pub fn part_1(input: &str) -> String {
    let grid = parse_grid(input);
    let mut counter = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell && removable(x, y, &grid) {
                counter += 1;
            }
        }
    }

    counter.to_string()
}

pub fn part_2(input: &str) -> String {
    let mut grid = parse_grid(input);
    let mut counter = 0;

    let mut queue = vec![];

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell {
                queue.push((x, y));
            }
        }
    }

    loop {
        let Some((x, y)) = queue.pop() else {
            break;
        };

        if roll_at(x, y, &grid) && removable(x, y, &grid) {
            counter += 1;
            grid[y][x] = false;

            for (x, y) in adjacent(x, y) {
                if roll_at(x, y, &grid) {
                    queue.push((x, y));
                }
            }
        }
    }

    counter.to_string()
}
