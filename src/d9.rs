use rayon::prelude::*;

type Point = (u64, u64);
type Polygon = Vec<Point>;

fn parse(input: &str) -> Polygon {
    let mut output = vec![];

    for line in input.lines() {
        let (left, right) = line.split_once(',').unwrap();
        let x: u64 = left.parse().unwrap();
        let y: u64 = right.parse().unwrap();
        output.push((x, y));
    }

    output
}

fn area(a: Point, b: Point) -> u64 {
    let dx = a.0.abs_diff(b.0) + 1;
    let dy = a.1.abs_diff(b.1) + 1;
    dx * dy
}

fn max_area(tiles: Vec<Point>) -> u64 {
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

fn max_area2(tiles: Vec<Point>) -> u64 {
    let mut max_found = 0;

    for a in 0..tiles.len() {
        max_found = max_found.max(
            ((a + 1)..tiles.len())
                .into_par_iter()
                .flat_map(|b| {
                    let tile_a = tiles[a];
                    let tile_b = tiles[b];
                    let value = area(tile_a, tile_b);

                    if valid_rectangle(tile_a, tile_b, &tiles) {
                        Some(value)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0),
        );
    }

    max_found
}

fn on_bounds(point: Point, polygon: &Polygon) -> bool {
    if polygon.len() < 4 {
        return false;
    }
    let (x, y) = point;
    let length = polygon.len();
    for index in 0..length {
        let p1 = polygon[index];
        let p2 = polygon[(index + 1) % length];
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        let x_min = x1.min(x2);
        let x_max = x1.max(x2);
        let y_min = y1.min(y2);
        let y_max = y1.max(y2);
        let x_line = x_min..=x_max;
        let y_line = y_min..=y_max;

        // Is inside by being on a red tile
        if point == p1 || point == p2 {
            return true;
        }

        // Is inside by being between two red tiles on y-axis
        if x1 == x2 && x == x1 && y_line.contains(&y) {
            return true;
        }

        if y1 == y2 && y == y1 && x_line.contains(&x) {
            return true;
        }
    }

    false
}

// Ray marching kind of
fn inside(point: Point, polygon: &Polygon) -> bool {
    if on_bounds(point, polygon) {
        return true;
    }

    let (x, y) = point;
    let length = polygon.len();

    let mut intersections = 0;

    for index in 0..length {
        let p1 = polygon[index];
        let p2 = polygon[(index + 1) % length];
        let (x1, y1) = p1;
        let (x2, y2) = p2;

        if x1 == x2 {
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);

            let edge_is_right_of = x1 > x;
            let is_y_aligned = (y_min..y_max).contains(&y);
            if edge_is_right_of && is_y_aligned {
                intersections += 1;
            }
        }
    }

    intersections % 2 != 0
}

fn valid_rectangle(a: Point, b: Point, polygon: &Polygon) -> bool {
    let (x1, y1) = a;
    let (x2, y2) = b;
    let x_min = x1.min(x2);
    let y_min = y1.min(y2);
    let x_max = x1.max(x2);
    let y_max = y1.max(y2);

    // for y in y_min..=y_max {
    //     for x in x_min..=x_max {
    //         if !inside((x, y), polygon) {
    //             return false;
    //         }
    //     }
    // }
    for y in y_min..=y_max {
        if !inside((x_min, y), polygon) || !inside((x_max, y), polygon) {
            return false;
        }
    }
    for x in x_min..=x_max {
        if !inside((x, y_min), polygon) || !inside((x, y_max), polygon) {
            return false;
        }
    }

    true
}

pub fn part_1(input: &str) -> String {
    let tiles = parse(input);
    max_area(tiles).to_string()
}

pub fn part_2(input: &str) -> String {
    let polygon = parse(input);

    max_area2(polygon).to_string()
}
