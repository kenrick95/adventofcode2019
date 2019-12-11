use super::program::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, PartialEq)]
enum Color {
    White,
    Black,
}
#[derive(Debug, Clone)]
struct RobotState {
    direction: Direction,
    // color: Color,
    /**
     * (y,x)
     */
    location: (i128, i128),
}

fn run_robot(
    starting_state: &State,
    starting_map: &HashMap<(i128, i128), Color>,
) -> (HashSet<(i128, i128)>, HashMap<(i128, i128), Color>) {
    // : HashMap<(i128, i128), Color>
    let map = RefCell::new(starting_map.clone());
    // (y, x); : HashSet<(i128, i128)>
    let painted_locations = RefCell::new(HashSet::new());

    let robot_state = RefCell::new(RobotState {
        direction: Direction::Up,
        location: (0, 0),
    });

    let output_counter = RefCell::new(0);
    let new_state = run_program(
        starting_state.positions.clone(),
        starting_state.program_counter.clone(),
        || {
            let current_robot_state = robot_state.borrow().clone();
            let current_map = map.borrow().clone();
            // Get "color" from `map`
            let color = current_map
                .get(&current_robot_state.location)
                .unwrap_or(&Color::Black);
            match color {
                Color::Black => 0,
                Color::White => 1,
            }
        },
        |result: i128| {
            let mut current_map = map.borrow().clone();
            let mut current_robot_state = robot_state.borrow().clone();
            let mut current_painted_locations = painted_locations.borrow().clone();
            let current_output_count: u128 = output_counter.borrow().clone();
            // Program will output twice! First output is next_color; second output is next_direction.
            if current_output_count % 2 == 0 {
                let next_color = match result {
                    0 => Color::Black,
                    1 => Color::White,
                    _ => Color::Black,
                };
                // Paint the current location
                current_map.insert(current_robot_state.location, next_color);
                // Push the current location to `painted_locations`
                current_painted_locations.insert(current_robot_state.location);
            } else {
                let next_direction = match result {
                    0 => {
                        // 90deg left
                        match current_robot_state.direction {
                            Direction::Up => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Down => Direction::Right,
                            Direction::Right => Direction::Up,
                        }
                    }
                    1 => {
                        // 90deg right
                        match current_robot_state.direction {
                            Direction::Up => Direction::Right,
                            Direction::Left => Direction::Up,
                            Direction::Down => Direction::Left,
                            Direction::Right => Direction::Down,
                        }
                    }
                    _ => Direction::Up,
                };
                current_robot_state.direction = next_direction;
                // After robot "turns", it should move forward exactly once
                let (dy, dx) = match current_robot_state.direction {
                    Direction::Up => (-1, 0),
                    Direction::Left => (0, 1),
                    Direction::Down => (1, 0),
                    Direction::Right => (0, -1),
                };
                current_robot_state.location = (
                    current_robot_state.location.0 + dy,
                    current_robot_state.location.1 + dx,
                );
            }

            output_counter.replace(current_output_count + 1);
            robot_state.replace(current_robot_state);
            map.replace(current_map);
            painted_locations.replace(current_painted_locations);
        },
        |_state| false,
    );

    let final_painted_locations = painted_locations.borrow().clone();
    let final_painted_colors = map.borrow().clone();
    println!("painted_locations: {:?}", final_painted_locations);
    println!("Answer part 1: {:?}", final_painted_locations.len());
    (final_painted_locations, final_painted_colors)
}

fn part1(positions: Vec<i128>) {
    let starting_state = State {
        positions: positions.clone(),
        relative_base: 0,
        program_counter: 0,
    };
    run_robot(&starting_state, &HashMap::new());
}

fn part2(positions: Vec<i128>) {
    let starting_state = State {
        positions: positions.clone(),
        relative_base: 0,
        program_counter: 0,
    };
    let mut robot_map = HashMap::new();
    robot_map.insert((0, 0), Color::White);
    let (painted_locations, painted_colors) = run_robot(&starting_state, &robot_map);
    const OFFSET: usize = 50;

    let mut print_map = vec![vec![Color::Black; OFFSET * 2]; OFFSET * 2];
    for i in 0..(OFFSET * 2) {
        for j in 0..(OFFSET * 2) {
            let y: i128 = i as i128 - OFFSET as i128;
            let x: i128 = j as i128 - OFFSET as i128;
            if painted_colors.contains_key(&(y, x)) {
                print_map[i][j] = painted_colors.get(&(y, x)).unwrap().clone();
            }
        }
    }
    for i in 0..(OFFSET * 2) {
        for j in 0..(OFFSET * 2) {
            if print_map[i][j] == Color::Black {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!(""); 
    }
    // LBJHEKLH printed out in reverse ?!

}

pub fn main() {
    let positions = super::utils::get_list_of_numbers::<i128>();
    println!("Positions {:?}", positions);

    part1(positions.clone());
    part2(positions.clone());
}
