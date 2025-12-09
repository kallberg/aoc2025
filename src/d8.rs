type Circuit = Vec<usize>;

type JunctionBox = (u64, u64, u64);

fn absorb(from: &mut Circuit, to: &mut Circuit) {
    to.extend(from.clone());
    from.clear();
}

fn merge_circuits(a: &mut Circuit, b: &mut Circuit) {
    if b.len() >= a.len() {
        absorb(a, b);
    } else {
        absorb(b, a);
    }
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
        let a_index = circuit_of(left, &circuits);
        let b_index = circuit_of(right, &circuits);

        if a_index == b_index {
            continue;
        }

        let mut a = circuits[a_index].clone();
        let mut b = circuits[b_index].clone();

        println!("pair {left}, {right} is merging circuit {a_index} with {b_index}");
        merge_circuits(&mut a, &mut b);

        circuits[a_index] = a;
        circuits[b_index] = b;
    }

    circuits.retain(|circuit| !circuit.is_empty());
    circuits.sort_by_key(|circuit| circuit.len());
    circuits.reverse();

    let mut product = 1;
    let circuit_count = circuits.len();

    for circuit in circuits.into_iter().take(3) {
        let value = circuit.len();
        println!("size={value} {circuit:?}");
        product *= value;
    }

    println!("total circuits: {circuit_count}");

    product.to_string()
}

pub fn part_2(_input: &str) -> String {
    String::new()
}
