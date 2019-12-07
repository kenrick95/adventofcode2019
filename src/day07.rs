use super::program::*;
use std::cell::RefCell;
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
pub fn main() {
    let positions: Vec<i32> = super::utils::get_string_from_stdio()
        .trim()
        .split(",")
        .map(|val| val.parse().unwrap())
        .collect();
    println!("Positions {:?}", positions);

    let num_programs = 5;
    let configs = Configs::new(num_programs);
    let mut max_last_result = std::i32::MIN;
    let mut configs_for_max_last_result = vec![0, 1, 2, 3, 4];
    for config in configs {
        // let program_results = Arc::new(Mutex::new(Vec::new()));
        let program_results = RefCell::new(vec![0; num_programs]);
        for i in 0..num_programs {
            let is_first_input = RefCell::new(true);
            run_program(
                positions.clone(),
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
                |result: i32| {
                    // println!("Output: {:?}", result);
                    // program_results.lock().unwrap().push(result);
                    let mut temp = program_results.borrow().clone();
                    temp[i] = result;
                    program_results.replace(temp);
                },
            );
        }
        max_last_result = i32::max(max_last_result, program_results.borrow()[num_programs - 1]);
        configs_for_max_last_result = config.clone();
    }

    println!(
        "configs_for_max_last_result: {:?}",
        configs_for_max_last_result
    );
    println!("max_last_result: {:?}", max_last_result);
}

#[derive(PartialEq, Debug)]
struct Configs {
    configs: Vec<i32>,
    count: usize,
}

impl Configs {
    fn new(size: usize) -> Configs {
        Configs {
            configs: vec![0; size],
            count: 0,
        }
    }
}

// TODO: Bug here, `44444` is not valid. Need to implement a real next_permutation
impl Iterator for Configs {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        let space: usize = 5;
        if self.count == 1 {
            Some(self.configs.clone())
        } else if self.count <= space.pow(self.configs.len() as u32) {
            let mut i = 0;
            loop {
                if self.configs[i] + 1 < space as i32 {
                    self.configs[i] += 1;
                    break;
                } else {
                    self.configs[i] = 0;
                    i += 1;
                }
            }

            Some(self.configs.clone())
        } else {
            None
        }
    }
}
