fn main() {
    println!("{}", aoc2025::d12::part_1(aoc2025::input::D12E));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_passes_example() {
        assert_eq!(aoc2025::d12::part_1(aoc2025::input::D12E), "2")
    }
}
