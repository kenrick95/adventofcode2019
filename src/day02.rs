use super::program::*;

fn run_program_with_tweak(positions: &Vec<i128>, a: i128, b: i128) -> State {
    let mut temp = positions.clone();
    temp[1] = a;
    temp[2] = b;
    return run_program(
        temp,
        0,
        0,
        || super::utils::get_number_from_stdio::<i128>().unwrap(),
        |result: i128| {
            println!("Output: {:?}", result);
        },
        |_state| false,
    );
}

pub fn main() {
    let positions = super::utils::get_list_of_numbers::<i128>();
    println!("Positions {:?}", positions);

    println!(
        "Part 1 answer {:?}",
        run_program_with_tweak(&positions, 12, 2).positions[0]
    );

    {
        let mut found = false;
        for a in 0..99 {
            for b in 0..99 {
                if run_program_with_tweak(&positions, a, b).positions[0] == 19690720 {
                    found = true;
                    println!("Part 2 answer {:?}", a * 100 + b);
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            println!("Part 2 not found!?");
        }
    }
}
