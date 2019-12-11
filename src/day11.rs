use super::program::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
enum Color {
    White,
    Black,
}
struct RobotState {
    direction: Direction,
    // color: Color,
    /**
     * (y,x)
     */
    location: (i128, i128),
}

fn part1(positions: Vec<i128>) {
    let map: HashMap<(i128, i128), Color> = HashMap::new();
    // (y, x)
    let painted_locations: HashSet<(i128, i128)> = HashSet::new();

    let robot_state = RobotState {
        direction: Direction::Up,
        location: (0, 0),
    };

    let mut current_state = State {
        positions: positions.clone(),
        relative_base: 0, // TODO: pass relative_base too?
        program_counter: 0,
    };

    let program_result = RefCell::new(0);
    let mut iteration = 0;
    loop {
        iteration += 1;
        let new_state = run_program(
            current_state.positions.clone(),
            current_state.program_counter.clone(),
            || {
                // Get "color" from `map`
                let color = map.get(&robot_state.location).unwrap_or(&Color::Black);
                match color {
                    Color::Black => 0,
                    Color::White => 1,
                }
            },
            |result: i128| {
                program_result.replace(result);
                // TODO: Program will output twice! First output is next_color; second output is next_direction.

                // TODO: Modify robot_state based on this `result`
                let next_color = match result {
                    0 => Color::Black,
                    1 => Color::White,
                    _ => Color::Black,
                };
                // TODO: Paint the current location
                // TODO: Push the current location to `painted_locations`

                let next_direction = match result {
                    0 => {
                        // 90deg left
                        match robot_state.direction {
                            Direction::Up => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Down => Direction::Right,
                            Direction::Right => Direction::Up,
                        }
                    }
                    1 => {
                        // 90deg right
                        match robot_state.direction {
                            Direction::Up => Direction::Right,
                            Direction::Left => Direction::Up,
                            Direction::Down => Direction::Left,
                            Direction::Right => Direction::Down,
                        }
                    },
                    _ => {
                        Direction::Up
                    }

                    // TODO: After robot "turns", it should move forward exactly once
                };
            },
            |_state| false, // TODO: Should we "halt" this program ever? :thinking:
        );

        if get_operation(&new_state).opcode == OpCode::Halt {
            break;
        }

        current_state = new_state;
    }

    println!("painted_locations: {:?}", painted_locations);
    println!("Answer part 1: {:?}", painted_locations.len());
}

pub fn main() {
    let positions = super::utils::get_list_of_numbers::<i128>();
    println!("Positions {:?}", positions);

    part1(positions.clone());
}
