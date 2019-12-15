use super::program::*;
use console::{style, Term};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::Duration;

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
    let positions = super::utils::get_list_of_numbers_from_file::<i128>("./src/day15-real.log");
    // println!("Positions {:?}", positions);
    part1(&positions.clone());
    // part2(&positions.clone());
}

fn part1(positions: &[i128]) {
    let map: RefCell<HashMap<(i128, i128), Cell>> = RefCell::new(HashMap::new());
    // (y,x)
    let location: RefCell<(i128, i128)> = RefCell::new((0, 0));

    // Current idea: Just like MDP
    // 1. Exploration (explore map as much as possible)
    //    - Left-wall hugging?
    //    - Have a "target" for unexplored cell so that it won't be stuck
    // 2. Race (BFS from start point to Target)

    // Another idea:
    // BFS at the "program" level:
    // from [reddit](https://www.reddit.com/r/adventofcode/comments/eaw4ua/2019_day_15_visualization_of_day_15_using_bfs/)
    // At each step, run "program" at different states

    // Exploration phase
    let next_direction: RefCell<Direction> = RefCell::new(Direction::Up);
    let next_location: RefCell<(i128, i128)> = RefCell::new((1, 0));
    run_program(
        positions.to_owned(),
        0,
        || {
            let direction = next_direction.borrow().clone() as i128;
            // println!("direction: {:?}", direction);
            print_map(
                &*map.borrow(),
                next_direction.borrow().clone(),
                location.borrow().clone(),
            );
            direction
        },
        |result: i128| {
            // println!("Output: {:?}", result);
            let next_cell_type = get_cell_type(result);
            (*map.borrow_mut()).insert(next_location.borrow().clone(), next_cell_type.clone());

            let current_direction = next_direction.borrow().clone();
            let current_location = if next_cell_type == Cell::Wall {
                location.borrow().clone()
            } else {
                next_location.borrow().clone()
            };
            // println!("Output: {:?} is {:?}", next_loc, next_cell_type);

            let new_direction =
                get_other_next_dir(&*map.borrow(), current_direction, current_location);

            
            print_map(
                &*map.borrow(),
                next_direction.borrow().clone(),
                current_location.clone(),
            );

            // let term = Term::stdout();
            // let input = term.read_key().unwrap();
            // let new_direction = match input {
            //     console::Key::ArrowLeft => Direction::Left,
            //     console::Key::ArrowRight => Direction::Right,
            //     console::Key::ArrowUp => Direction::Up,
            //     console::Key::ArrowDown => Direction::Down,
            //     _ => Direction::Up,
            // };

            // Commit direction and location
            let new_delta = get_deltas(&new_direction);
            location.replace(current_location);
            next_location.replace((
                current_location.0 + new_delta.0,
                current_location.1 + new_delta.1,
            ));
            next_direction.replace(new_direction);

            
        },
        |_state| false, // TODO: Halt if we've found Cell::Target
    );
}

fn get_cell_type(raw_result: i128) -> Cell {
    match raw_result {
        0 => Cell::Wall,
        1 => Cell::Empty,
        2 => Cell::Target,
        _ => Cell::Empty,
    }
}

fn get_deltas(direction: &Direction) -> (i128, i128) {
    match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn get_other_next_dir(
    map: &HashMap<(i128, i128), Cell>,
    current_direction: Direction,
    current_location: (i128, i128),
) -> Direction {
    //
    match current_direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}
fn print_map(
    map: &HashMap<(i128, i128), Cell>,
    current_direction: Direction,
    current_location: (i128, i128),
) {
    let offset: i128 = 20;
    let term = Term::stdout();
    // println!("{:?}", map);
    term.clear_screen();
    for (location, tile) in map.iter() {
        term.move_cursor_to(
            (location.1 + offset) as usize,
            (location.0 + offset) as usize,
        );
        if location.1 == current_location.1 && location.0 == current_location.0 {
            term.write_str(&get_direction_as_str(&current_direction));
        } else {
            term.write_str(&get_cell_as_str(tile));
        }
    }
    thread::sleep(Duration::from_millis(20));
}

fn get_cell_as_str(cell: &Cell) -> String {
    let result = match cell {
        Cell::Empty => ".",
        Cell::Wall => "#",
        Cell::Target => "@",
    };
    result.to_string()
}

fn get_direction_as_str(direction: &Direction) -> String {
    let result = match direction {
        Direction::Up => "↑",
        Direction::Down => "↓",
        Direction::Left => "←",
        Direction::Right => "→",
    };
    result.to_string()
}
