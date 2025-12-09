fn main() {
    println!("{}", aoc2025::d8::part_1(aoc2025::input::D8, 1000));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1_example() {
        assert_eq!(aoc2025::d8::part_1(aoc2025::input::D8E, 10), "40")
    }
}
