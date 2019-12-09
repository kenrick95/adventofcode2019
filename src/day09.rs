use super::program::*;
pub fn main() {
    let positions = super::utils::get_list_of_numbers::<i128>();

    println!("Positions {:?}", positions);
    run_program(
        positions,
        0,
        || super::utils::get_number_from_stdio::<i128>().unwrap(),
        |result: i128| {
            println!("Output: {:?}", result);
        },
        |_state| false,
    );
}