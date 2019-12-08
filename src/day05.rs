use super::program::*;

pub fn main() {
    let positions: Vec<i32> = super::utils::get_string_from_stdio()
        .trim()
        .split(",")
        .map(|val| val.parse().unwrap())
        .collect();
    println!("Positions {:?}", positions);
    run_program(
        positions,
        0,
        || super::utils::get_number_from_stdio::<i32>().unwrap(),
        |result: i32| {
            println!("Output: {:?}", result);
        },
        |_state| false,
    );
}
