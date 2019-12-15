use super::program::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Wall = 0,
    Empty = 1,
    Target = 2,
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

pub fn main() {
    let positions = super::utils::get_list_of_numbers::<i128>();
    // println!("Positions {:?}", positions);
    part1(&positions.clone());
    // part2(&positions.clone());
}

fn part1(positions: &[i128]) {
    let map: RefCell<HashMap<(i128, i128), Cell>> = RefCell::new(HashMap::new());
    // (y,x)
    let location: RefCell<(i128, i128)> = RefCell::new((0, 0));

    // Exploration phase
    let next_direction: RefCell<Direction> = RefCell::new(Direction::Left);
    let next_location: RefCell<(i128, i128)> = RefCell::new((1, 0));
    run_program(
        positions.to_owned(),
        0,
        || {
            let direction = next_direction.borrow().clone() as i128;
            println!("direction: {:?}", direction);
            direction
        },
        |result: i128| {
            let next_cell_type = get_cell_type(result);
            println!("Output: {:?}", next_cell_type);
            let next_loc = next_location.borrow().clone();
            (*map.borrow_mut()).insert(next_loc, next_cell_type.clone());

            let next_dir = next_direction.borrow().clone();

            let new_direction = match next_cell_type {
                Cell::Wall => {
                    // Update direction
                    get_other_next_dir(next_dir) // TODO: Consider the "map" :thinking:
                }
                Cell::Empty => {
                    // Go forward
                    next_dir
                }
                Cell::Target => {
                    // Go forward
                    next_dir
                }
            };
            // Update location
            let new_delta = get_deltas(new_direction);
            location.replace(next_loc);
            next_location.replace((next_loc.0 + new_delta.0, next_loc.1 + new_delta.1));

        },
        |_state| false, // TODO: Halt if we've found Cell::Target
    );
}

fn get_cell_type(raw_result: i128) -> Cell {
    match raw_result {
        0 => Cell::Empty,
        1 => Cell::Wall,
        2 => Cell::Target,
        _ => Cell::Empty,
    }
}

fn get_deltas(direction: Direction) -> (i128, i128) {
    match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn get_other_next_dir(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}