use super::program::*;
use std::cell::RefCell;
use std::collections::HashSet;
// use std::sync::{Arc, Mutex};

/**
 * - There are 5 programs running in series
 * - Program is run using the raw "positions" codes from stdin
 * - Each program will have two OpCode::Input and one Opcode::Output
 * - Each program's first OpCode::Input is an integer k_i from 0 to 4 (inclusive)
 * - Each program's second OpCode::Input is the previous program's OpCode::Output
 * - First program's second OpCode::Input is 0
 * - Last program's second OpCode::Output is the ThursterSignal
 * - Find the sequences of k_i that maximizes ThrusterSignal
 * */
fn part1(positions: Vec<i128>) {
    let num_programs = 5;
    let configs = Configs::new(vec![0, 1, 2, 3, 4]);
    let mut max_last_result = std::i128::MIN;
    let mut configs_for_max_last_result = vec![0, 1, 2, 3, 4];
    for config in configs {
        // let program_results = Arc::new(Mutex::new(Vec::new()));
        let program_results = RefCell::new(vec![0; num_programs]);
        for i in 0..num_programs {
            let is_first_input = RefCell::new(true);
            run_program(
                positions.clone(),
                0,
                || {
                    let ifi = is_first_input.borrow().clone();
                    let program_results = program_results.borrow().clone();
                    if ifi {
                        is_first_input.replace(false);
                        config[i]
                    } else {
                        if i > 0 {
                            program_results[i - 1]
                        } else {
                            0
                        }
                    }
                },
                |result: i128| {
                    // println!("Output: {:?}", result);
                    // program_results.lock().unwrap().push(result);
                    let mut temp = program_results.borrow().clone();
                    temp[i] = result;
                    program_results.replace(temp);
                },
                |_state| false,
            );
        }
        max_last_result = i128::max(max_last_result, program_results.borrow()[num_programs - 1]);
        configs_for_max_last_result = config.clone();
    }
    println!(
        "configs_for_max_last_result: {:?}",
        configs_for_max_last_result
    );
    println!("max_last_result: {:?}", max_last_result);
}

/**
 * - There are 5 programs running in series
 * - Program is run using the raw "positions" codes from stdin
 * - Each program will have two OpCode::Input and one Opcode::Output
 * - Each program's first OpCode::Input is an integer k_i from 0 to 4 (inclusive)
 * - Each program's second (and onwards) OpCode::Input is the previous program's OpCode::Output
 * - First program's second OpCode::Input is 0
 * - Last program's second (and onwards) OpCode::Output is the third (and onwards) OpCode::Input for the First program
 * - Last program's final OpCode::Output is the ThursterSignal
 * - Find the sequences of k_i that maximizes ThrusterSignal
 * */
fn part2(positions: Vec<i128>) {
    let num_programs = 5;
    let configs = Configs::new(vec![5, 6, 7, 8, 9]);
    let mut max_last_result = std::i128::MIN;
    let mut configs_for_max_last_result = vec![5, 6, 7, 8, 9];
    for config in configs {
        let program_results = RefCell::new(vec![0; num_programs]);
        let mut iteration = 0;
        let mut program_states = vec![];
        for _i in 0..num_programs {
            program_states.push(State {
                positions: positions.clone(),
                relative_base: 0,
                program_counter: 0,
            })
        }
        while iteration < 1000 {
            for i in 0..num_programs {
                let is_first_input = RefCell::new(iteration == 0);
                let has_produced_output = RefCell::new(false);
                let new_state = run_program(
                    program_states[i].positions.clone(),
                    program_states[i].program_counter,
                    || {
                        let ifi = is_first_input.borrow().clone();
                        let program_results = program_results.borrow().clone();
                        if ifi {
                            is_first_input.replace(false);
                            config[i]
                        } else {
                            if i > 0 {
                                program_results[i - 1]
                            } else {
                                program_results[num_programs - 1]
                            }
                        }
                    },
                    |result: i128| {
                        // println!("Output: {:?}", result);
                        // program_results.lock().unwrap().push(result);
                        let mut temp = program_results.borrow().clone();
                        temp[i] = result;
                        program_results.replace(temp);
                        has_produced_output.replace(true);
                    },
                    |_state| has_produced_output.borrow().clone(),
                );
                program_states[i] = new_state;
            }
            // println!(
            //     "Iteration {:?}: program_results {:?}",
            //     iteration,
            //     program_results.borrow()
            // );
            if get_operation(&program_states[num_programs - 1]).opcode == OpCode::Halt {
                break;
            }
            iteration += 1;
        }
        max_last_result = i128::max(max_last_result, program_results.borrow()[num_programs - 1]);
        configs_for_max_last_result = config.clone();
    }
    println!(
        "configs_for_max_last_result: {:?}",
        configs_for_max_last_result
    );
    println!("max_last_result: {:?}", max_last_result);
}

pub fn main() {
    let positions: Vec<i128> = super::utils::get_string_from_stdio()
        .trim()
        .split(",")
        .map(|val| val.parse().unwrap())
        .collect();
    println!("Positions {:?}", positions);

    part1(positions.clone());
    part2(positions.clone());
}

#[derive(PartialEq, Debug)]
struct Configs {
    initial_state: Vec<i128>,
    index: usize,
}

impl Configs {
    fn new(initial_state: Vec<i128>) -> Configs {
        Configs {
            initial_state: initial_state,
            index: 0,
        }
    }
}

impl Iterator for Configs {
    type Item = Vec<i128>;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.initial_state.len();
        if self.index < factorial(len) {
            let mut result = vec![0; len];

            let mut digits: HashSet<i128> = HashSet::new();
            for number in self.initial_state.clone() {
                digits.insert(number);
            }

            for i in 0..len {
                let mut digits_sorted: Vec<i128> = digits.clone().into_iter().collect();
                digits_sorted.sort();
                let chosen =
                    digits_sorted[(self.index % factorial(len - i)) / factorial(len - i - 1)];
                digits.remove(&chosen);
                result[i] = chosen;
            }

            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

// NOTE: Try to memoize this
fn factorial(number: usize) -> usize {
    if number <= 1 {
        return 1;
    } else {
        return number * factorial(number - 1);
    }
}
