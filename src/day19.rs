use super::program::*;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn main() {
    let positions = super::utils::get_list_of_numbers_from_file::<i128>("./src/day19-real.log");
    part1(&positions);
}

fn get_result_on_location(raw_positions: &[i128], location: (i128, i128)) -> i128 {
    let y = location.0;
    let x = location.1;
    let input_count: RefCell<usize> = RefCell::new(0);
    let final_result: RefCell<i128> = RefCell::new(0);
    run_program(
        raw_positions.to_owned(),
        0,
        0,
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
            final_result.replace(result);
        },
        |_state| {
            if *input_count.borrow() == 2 {
                true
            } else {
                false
            }
        },
    );
    let returned_result = *final_result.borrow();
    returned_result.clone()
}

fn part1(positions: &[i128]) {
    // let map: RefCell<HashMap<(usize, usize), bool>> = RefCell::new(HashMap::new());
    let mut answer = 0;
    for y in 0..50 {
        for x in 0..50 {
            answer += get_result_on_location(positions, (y,x));
            // (*map.borrow_mut()).insert(
            //     (y as usize, x as usize),
            //     if result == 1 { true } else { false },
            // );
        }
    }

    println!("Answer part 1: {:?}", answer);
}
