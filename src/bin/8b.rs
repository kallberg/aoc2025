fn main() {
    println!("{}", aoc2025::d8::part_2(aoc2025::input::D8));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_2_example() {
        assert_eq!(aoc2025::d8::part_2(aoc2025::input::D8E), "25272")
    }
}
