use super::program::*;
pub fn main() {
    let positions = super::utils::get_list_of_numbers();

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