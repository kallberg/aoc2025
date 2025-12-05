mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod input;

fn main() {
    let d1_1 = d1::part_1(input::D1);
    let d1_2 = d1::part_2(input::D1);
    let d2_1 = d2::part_1(input::D2);
    let d2_2 = d2::part_2(input::D2);
    let d3_1 = d3::part_1(input::D3);
    let d3_2 = d3::part_2(input::D3);
    let d4_1 = d4::part_1(input::D4);
    let d4_2 = d4::part_2(input::D4);
    let d5_1 = d5::part_1(input::D5);
    let d5_2 = d5::part_2(input::D5);

    println!("{d1_1}");
    println!("{d1_2}");
    println!("{d2_1}");
    println!("{d2_2}");
    println!("{d3_1}");
    println!("{d3_2}");
    println!("{d4_1}");
    println!("{d4_2}");
    println!("{d5_1}");
    println!("{d5_2}");
}
