use super::program::*;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn main() {
    let positions = super::utils::get_list_of_numbers_from_file::<i128>("./src/day19-real.log");
    part1(&positions);
}
fn part1(positions: &[i128]) {
    let map: RefCell<HashMap<(usize, usize), bool>> = RefCell::new(HashMap::new());

    let mut current_positons = positions.to_owned();
    let mut current_program_counter = 0;
    let mut current_relative_base = 0;
    let answer: RefCell<i128> = RefCell::new(0);

    for y in 0..50 {
        for x in 0..50 {
            let location = (y, x);
            let input_count: RefCell<usize> = RefCell::new(0);
            // println!("a {} {}", x, y);
            let new_state = run_program(
                current_positons.clone(),
                current_program_counter,
                current_relative_base,
                || {
                    // println!("input {} {}", x, y);
                    if *input_count.borrow() == 0 {
                        *input_count.borrow_mut() += 1;
                        x
                    } else {
                        y
                    }
                },
                |result: i128| {
                    println!("res {}", result);
                    *answer.borrow_mut() += result;

                    (*map.borrow_mut()).insert(
                        (y as usize, x as usize),
                        if result == 1 { true } else { false },
                    );
                },
                |_state| {
                    if *input_count.borrow() == 2 {
                        true
                    } else {
                        false
                    }
                },
            );

            // current_positons = new_state.positions;
            // current_program_counter = new_state.program_counter;
            // current_relative_base = new_state.relative_base;
        }
    }

    println!("Answer part 1: {:?}", answer);
}
