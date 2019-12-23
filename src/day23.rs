use super::program::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn main() {
    let positions = super::utils::get_list_of_numbers_from_file::<i128>("./src/day23-real.log");
    part1(&positions);

    // NOTE: Part 2 is more troublesome,
    // Need a "NAT" computer that sits on address 255 with memory capacity of 2 (one for "x" and one for "y")
    // Need a network monitoring to determine if they are idle
    // Idle if
    // - all computers have buffer_input len == 0
    // - repeatedly (how many?) requesting for input without sending out output
    // If idle, "NAT" computer puts the x&y to computer of address=0; and repeat the whole mess
    // Also need to keep track of the output of "NAT" to computer of address=0; if the "y" valuce repeats twice in a row, break and prints this value --> answer for part 2
}

/**
 * For each iteration:
 * - run_program
 * - if input is requested, pop from its buffer_input
 * - if output is generated, push to buffer_output
 * */
fn part1(positions: &[i128]) {
    let num_computers = 50;
    let buffer_input: RefCell<HashMap<usize, VecDeque<i128>>> = RefCell::new(HashMap::new());
    let buffer_output: RefCell<HashMap<usize, VecDeque<i128>>> = RefCell::new(HashMap::new());

    let mut program_states = vec![];

    for i in 0..num_computers {
        (*buffer_input.borrow_mut()).insert(i, VecDeque::from(vec![i as i128]));
        (*buffer_output.borrow_mut()).insert(i, VecDeque::from(vec![]));

        program_states.push(State {
            positions: positions.to_owned(),
            relative_base: 0,
            program_counter: 0,
        })
    }

    let mut it = 0;
    while it < 100_000 {
        it += 1;
        println!("it {}", it);

        for i in 0..num_computers {
            let new_states = run_program(
                program_states[i].positions.clone(),
                program_states[i].program_counter,
                program_states[i].relative_base,
                || {
                    // println!("[{}] Input", i);
                    let mut mutable_buffer_input = buffer_input.borrow_mut();
                    let current_input_queue = mutable_buffer_input.get_mut(&i).unwrap();
                    let input = if current_input_queue.is_empty() {
                        -1
                    } else {
                        current_input_queue.pop_front().unwrap()
                    };
                    println!("[{}][{}] Input = {}", it, i, input);
                    input
                },
                |result: i128| {
                    // println!("[{}] Output", i);
                    let mut mutable_buffer_output = buffer_output.borrow_mut();
                    let current_output_queue = mutable_buffer_output.get_mut(&i).unwrap();
                    println!("[{}][{}] Output = {}", it, i, result);
                    current_output_queue.push_back(result);
                },
                |_state| true,
            );

            program_states[i] = new_states;
        }
        for i in 0..num_computers {
            // println!("[{}][{}] Post-process", it, i);
            let mut mutable_buffer_output = buffer_output.borrow_mut();
            let mut mutable_buffer_input = buffer_input.borrow_mut();
            let current_output_queue = mutable_buffer_output.get_mut(&i).unwrap();

            if current_output_queue.len() == 3 {
                let destination_address = current_output_queue.pop_front().unwrap() as usize;
                let current_output_x = current_output_queue.pop_front().unwrap();
                let current_output_y = current_output_queue.pop_front().unwrap();

                if destination_address >= num_computers {
                    println!("x {:?}; y {:?}", current_output_x, current_output_y);
                    // Found the answer! Break out!
                    it = 100_000;
                    break;
                }

                let destination_input_queue =
                    mutable_buffer_input.get_mut(&destination_address).unwrap();
                destination_input_queue.push_back(current_output_x);
                destination_input_queue.push_back(current_output_y);
            }
        }
    }
}
