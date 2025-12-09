type Circuit = Vec<usize>;

type JunctionBox = (u64, u64, u64);

fn merge(into: &mut Circuit, from: &Circuit) {
    into.extend(from.clone());
}

fn distance_squared(a: JunctionBox, b: JunctionBox) -> u64 {
    let (x1, y1, z1) = a;
    let (x2, y2, z2) = b;
    let dx = x1.abs_diff(x2);
    let dy = y1.abs_diff(y2);
    let dz = z1.abs_diff(z2);
    dx.pow(2) + dy.pow(2) + dz.pow(2)
}

fn parse_junction_boxes(input: &str) -> Vec<JunctionBox> {
    let mut output = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.splitn(3, ',').collect();
        let left = parts[0];
        let middle = parts[1];
        let right = parts[2];

        let x = left.parse().unwrap();
        let y = middle.parse().unwrap();
        let z = right.parse().unwrap();

        output.push((x, y, z));
    }
    output
}

fn id_pairs_sorted(boxes: Vec<JunctionBox>) -> Vec<(usize, usize)> {
    let count = boxes.len();
    let mut output = vec![];
    for left in 0..count {
        let right_start = left + 1;
        for right in right_start..count {
            let a = boxes[left];
            let b = boxes[right];
            let distance = distance_squared(a, b);
            output.push(((left, right), distance));
        }
    }
    output.sort_unstable_by_key(|a| a.1);

    // Distance not needed once it is sorted
    output.into_iter().map(|a| a.0).collect()
}

fn circuit_of(box_id: usize, circuits: &[Circuit]) -> usize {
    for (index, circuit) in circuits.iter().enumerate() {
        if circuit.contains(&box_id) {
            return index;
        }
    }
    panic!("Lost track of box with id {box_id}");
}

pub fn part_1(input: &str, max: usize) -> String {
    let boxes = parse_junction_boxes(input);
    let circuit_count = boxes.len();
    let mut sorted = id_pairs_sorted(boxes);
    sorted.truncate(max);
    let mut circuits = vec![];

    for index in 0..circuit_count {
        circuits.push(vec![index])
    }

    for pair in sorted {
        let (left, right) = pair;
        let merge_into = circuit_of(left, &circuits);
        let merge_from = circuit_of(right, &circuits);

        if merge_into == merge_from {
            continue;
        }

        let source = circuits[merge_from].clone();
        let target = &mut circuits[merge_into];

        merge(target, &source);

        circuits[merge_from].clear();
    }

    circuits.retain(|circuit| !circuit.is_empty());
    circuits.sort_by_key(|circuit| circuit.len());
    circuits.reverse();

    let mut product = 1;

    for circuit in circuits.into_iter().take(3) {
        let value = circuit.len();
        product *= value;
    }

    product.to_string()
}

pub fn part_2(input: &str) -> String {
    let boxes = parse_junction_boxes(input);
    let circuit_count = boxes.len();
    let sorted = id_pairs_sorted(boxes.clone());
    let mut circuits = vec![];
    let mut remaining: Vec<usize> = (0..circuit_count).collect();

    for index in 0..circuit_count {
        circuits.push(vec![index])
    }

    for pair in sorted {
        let (left, right) = pair;
        let merge_into = circuit_of(left, &circuits);
        let merge_from = circuit_of(right, &circuits);

        if merge_into == merge_from {
            continue;
        }

        remaining.retain(|index| *index != merge_from);
        let source = circuits[merge_from].clone();
        let target = &mut circuits[merge_into];

        merge(target, &source);

        circuits[merge_from].clear();

        if remaining.len() == 1 {
            let box1 = &boxes[left];
            let box2 = &boxes[right];
            let product = box1.0 * box2.0;
            return product.to_string();
        }
    }

    String::from("0HN0!")
}
