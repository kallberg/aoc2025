type Tile = (u64, u64);

fn parse(input: &str) -> Vec<Tile> {
    let mut output = vec![];

    for line in input.lines() {
        let (left, right) = line.split_once(',').unwrap();
        let x: u64 = left.parse().unwrap();
        let y: u64 = right.parse().unwrap();
        output.push((x, y));
    }

    output
}

fn area(a: Tile, b: Tile) -> u64 {
    let dx = a.0.abs_diff(b.0) + 1;
    let dy = a.1.abs_diff(b.1) + 1;
    dx * dy
}

fn max_area(tiles: Vec<Tile>) -> u64 {
    let mut max_found = 0;
    for a in 0..tiles.len() {
        for b in (a + 1)..tiles.len() {
            let tile_a = tiles[a];
            let tile_b = tiles[b];
            let value = area(tile_a, tile_b);
            max_found = max_found.max(value);
        }
    }
    max_found
}

pub fn part_1(input: &str) -> String {
    let tiles = parse(input);
    max_area(tiles).to_string()
}

pub fn part_2(_input: &str) -> String {
    String::new()
}
