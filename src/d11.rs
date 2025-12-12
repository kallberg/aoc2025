use std::collections::HashMap;

type DeviceMap = HashMap<String, Vec<String>>;

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut connections = HashMap::new();
    for line in input.lines() {
        let (device, rest) = line.split_once(": ").unwrap();
        let outputs = rest
            .split_whitespace()
            .map(|entry| entry.to_string())
            .collect();
        connections.insert(device.into(), outputs);
    }
    connections
}

fn connections(from: &str, to: &str, map: &DeviceMap, cache: &mut HashMap<String, usize>) -> usize {
    if from.eq(to) {
        return 1;
    }
    if let Some(cache_hit) = cache.get(from) {
        return *cache_hit;
    }
    let Some(next) = map.get(from) else {
        return 0;
    };
    let result = next
        .iter()
        .map(|from| connections(from, to, map, cache))
        .sum();
    cache.insert(from.to_string(), result);
    result
}

fn connections_with_required_stops(
    from: &str,
    dac: bool,
    fft: bool,
    to: &str,
    map: &DeviceMap,
    cache: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    if from.eq(to) {
        if dac && fft {
            return 1;
        }
        return 0;
    }

    if let Some(cache_hit) = cache.get(&(from.to_string(), dac, fft)) {
        return *cache_hit;
    }

    let mut next_dac = dac;
    let mut next_fft = fft;

    match from {
        "dac" => next_dac = true,
        "fft" => next_fft = true,
        _ => {}
    };

    let Some(next) = map.get(from) else {
        return 0;
    };

    let result = next
        .iter()
        .map(|from| connections_with_required_stops(from, next_dac, next_fft, to, map, cache))
        .sum();

    cache.insert((from.to_string(), dac, fft), result);

    return result;
}

pub fn part_1(input: &str) -> String {
    let map = parse(input);

    connections("you", "out", &map, &mut HashMap::new()).to_string()
}

pub fn part_2(input: &str) -> String {
    let map = parse(input);

    connections_with_required_stops("svr", false, false, "out", &map, &mut HashMap::new())
        .to_string()
}
