fn parse_instruction(input_str: &str) -> Option<(bool, u32)> {
    if input_str.len() < 1 {
        return None;
    }

    let (direction_string, distance_string) = input_str.split_at(1);

    let dir = match direction_string {
        "L" => false,
        "R" => true,
        _ => return None,
    };

    let Ok(distance) = distance_string.parse::<u32>() else {
        return None;
    };

    Some((dir, distance))
}

pub fn part_1(input_str: &str) -> String {
    let mut dial: i32 = 50;
    let mut counter = 0;

    for line in input_str.lines() {
        if line.is_empty() {
            continue;
        }
        let Some((direction, distance)) = parse_instruction(line) else {
            panic!("failed to parse line={line}");
        };
        match direction {
            false => dial -= distance as i32,
            true => dial += distance as i32,
        }
        dial %= 100;

        if dial == 0 {
            counter += 1
        }
    }

    counter.to_string()
}

pub fn part_2(input_str: &str) -> String {
    let mut dial: i32 = 50;
    let mut counter = 0;

    for line in input_str.lines() {
        let prev = dial;
        if line.is_empty() {
            continue;
        }
        let Some((direction, distance)) = parse_instruction(line) else {
            panic!("failed to parse line={line}");
        };
        let mod_distance = distance % 100;
        let rem_distance = distance - mod_distance;
        let distance_revolutions = rem_distance / 100;
        match direction {
            false => dial -= mod_distance as i32,
            true => dial += mod_distance as i32,
        }
        let unconstrained = dial;

        dial %= 100;

        if dial < 0 {
            dial += 100;
        }

        if dial == 0 || (prev != 0 && dial != unconstrained) {
            counter += 1;
        }
        counter += distance_revolutions;
    }

    counter.to_string()
}
