use super::program::*;

fn run_program_with_tweak(positions: &Vec<i32>, a: i32, b: i32) -> State {
    let mut temp = positions.clone();
    temp[1] = a;
    temp[2] = b;
    return run_program(
        temp,
        || super::utils::get_number_from_stdio::<i32>().unwrap(),
        |result: i32| {
            println!("Output: {:?}", result);
        },
    );
}

pub fn main() {
    let positions: Vec<i32> = super::utils::get_string_from_stdio()
        .trim()
        .split(",")
        .map(|val| val.parse().unwrap())
        .collect();
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
