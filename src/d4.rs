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

fn adjacent_rolls(x: usize, y: usize, grid: &Grid) -> usize {
    let range_x = x.saturating_sub(1)..=(x + 1);
    let range_y = y.saturating_sub(1)..=(y + 1);
    let mut counter = 0;

    for a_y in range_y {
        for a_x in range_x.clone() {
            if x == a_x && y == a_y {
                continue;
            }
            if roll_at(a_x, a_y, grid) {
                counter += 1;
            }
        }
    }

    counter
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
        // println!("row={}", row_str(row));
        for (x, cell) in row.iter().enumerate() {
            if *cell && adjacent_rolls(x, y, &grid) < 4 {
                // println!("x={x}, y={y} => moveable");
                counter += 1;
            }
        }
    }

    counter.to_string()
}

pub fn part_2(input: &str) -> String {
    String::new()
}
