mod d1;
mod d2;
mod input;

fn main() {
    let d1_1 = d1::part_1(input::D1);
    let d1_2 = d1::part_2(input::D1);
    let d2_1 = d2::part_1(input::D2);
    let d2_2 = d2::part_2(input::D2);

    println!("{d1_1}");
    println!("{d1_2}");
    println!("{d2_1}");
    println!("{d2_2}");
}
