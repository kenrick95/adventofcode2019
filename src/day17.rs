use super::program::*;
use std::cell::RefCell;
use std::collections::HashMap;

use console::Term;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Scaffold,
    OpenSpace,
    RobotOnScaffold,
    RobotOutsideScaffold,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

#[derive(Debug, Clone, PartialEq)]
struct Cell {
    cell_type: CellType,

    // if cell_type == RobotOnScaffold, robot_direction has meaning
    robot_direction: Direction,
}

pub fn main() {
    let positions = super::utils::get_list_of_numbers_from_file::<i128>("./src/day17-real.log");
    part1(&positions);

    part2(&positions);
    // positions[0] = 2;
    // Program input is i128
    // main movement routine (max 3 definitions)
    //    'A,A,B,C,B,C,B,C\n' // fn A, fn A, fn B, ...
    // each movement function (for each for the three)
    //    '10,L,8,R,6\n' // forward 10, turn left, forward 8, turn right, forward 6
    // continuous video feed
    //    'y' or 'n'      // resource intensive, use it only for debug?

    // main routine and movement functions may each contain at most 20 characters, not counting the newline.
}

fn part2(positions: &[i128]) {
    let mut new_positions = positions.to_owned();
    new_positions[0] = 2;

    // Computed by hand :imp:
    let solution = "A,B,B,A,C,A,C,A,C,B\n\
                    R,6,R,6,R,8,L,10,L,4\n\
                    R,6,L,10,R,8\n\
                    L,4,L,12,R,6,L,10\n\
                    n\n";
    let solution_chars = RefCell::new(solution.chars());

    run_program(
        new_positions.clone(),
        0,
        0,
        || (*solution_chars.borrow_mut()).next().unwrap() as i128,
        |result: i128| {
            if result > 255 {
                println!("Output: {:?}", result);
            }
        },
        |_state| false,
    );
}

fn part1(positions: &[i128]) {
    let map: RefCell<HashMap<(i128, i128), Cell>> = RefCell::new(HashMap::new());

    // (y,x)
    let location: RefCell<(i128, i128)> = RefCell::new((0, 0));
    run_program(
        positions.to_owned(),
        0,
        0,
        || 0,
        |result: i128| {
            // println!("Output: {:?}", result);
            let current_location = location.borrow();

            if result == 10 {
                // \n
                location.replace((current_location.0 + 1, 0));
            } else {
                let cell_type = match result {
                    35 => CellType::Scaffold,             // #
                    46 => CellType::OpenSpace,            // .
                    118 => CellType::RobotOnScaffold,     // v
                    94 => CellType::RobotOnScaffold,      // ^
                    60 => CellType::RobotOnScaffold,      // <
                    62 => CellType::RobotOnScaffold,      // >
                    88 => CellType::RobotOutsideScaffold, // X
                    _ => CellType::OpenSpace,
                };
                let robot_direction = if cell_type == CellType::RobotOnScaffold {
                    match result {
                        118 => Direction::Down, // v
                        94 => Direction::Up,    // ^
                        60 => Direction::Left,  // <
                        62 => Direction::Right, // >
                        _ => Direction::Up,
                    }
                } else {
                    Direction::Up
                };

                (*map.borrow_mut()).insert(
                    current_location.clone(),
                    Cell {
                        cell_type,
                        robot_direction,
                    },
                );

                location.replace((current_location.0, current_location.1 + 1));
            }
        },
        |_state| false,
    );

    print_map(&map.borrow().clone());

    // Find intersections
    {
        let intersections = get_intersections(&map.borrow());
        println!("intersections {:?}", intersections);
        let answer: i128 = intersections
            .iter()
            .map(|location| location.0 * location.1)
            .sum();
        println!("Answer part 1: {:?}", answer);
    }
}

fn get_intersections(map: &HashMap<(i128, i128), Cell>) -> Vec<(i128, i128)> {
    let mut result = vec![];

    for (location, cell) in map.iter() {
        if cell.cell_type == CellType::Scaffold {
            let cell_top = get_cell_type(&map, (location.0 - 1, location.1));
            let cell_bottom = get_cell_type(&map, (location.0 + 1, location.1));
            let cell_left = get_cell_type(&map, (location.0, location.1 - 1));
            let cell_right = get_cell_type(&map, (location.0, location.1 + 1));
            if cell_top == CellType::Scaffold
                && cell_bottom == CellType::Scaffold
                && cell_left == CellType::Scaffold
                && cell_right == CellType::Scaffold
            {
                result.push(*location);
            }
        }
    }

    result
}

fn get_cell_type(map: &HashMap<(i128, i128), Cell>, location: (i128, i128)) -> CellType {
    let cell_option = map.get(&location);
    if let Some(cell) = cell_option {
        cell.cell_type
    } else {
        CellType::OpenSpace
    }
}

fn print_map(map: &HashMap<(i128, i128), Cell>) {
    let offset: i128 = 0;
    let term = Term::stdout();
    // println!("{:?}", map);
    let mut max_x = 0;
    let mut max_y = 0;
    term.clear_screen();
    for (location, cell) in map.iter() {
        term.move_cursor_to(
            (location.1 + offset) as usize,
            (location.0 + offset) as usize,
        );
        max_x = std::cmp::max(max_x, location.1);
        max_y = std::cmp::max(max_y, location.0);
        term.write_str(&get_cell_as_str(cell));
    }
    term.move_cursor_to((max_x + offset) as usize, (max_y + offset) as usize);
    term.write_line("");
    term.write_line(&max_y.to_string());
    term.write_line(&max_x.to_string());
    // let input = term.read_key().unwrap();
}

fn get_cell_as_str(cell: &Cell) -> String {
    let result = match cell.cell_type {
        CellType::Scaffold => "#",
        CellType::OpenSpace => ".",
        CellType::RobotOnScaffold => match cell.robot_direction {
            Direction::Down => "v",
            Direction::Up => "^",
            Direction::Left => "<",
            Direction::Right => ">",
        },
        CellType::RobotOutsideScaffold => "X",
    };
    result.to_string()
}
