use super::program::*;
use console::{style, Term};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

#[derive(Debug, Clone, PartialEq)]
enum JoystickPosition {
    Neutral = 0,
    TiltLeft = -1,
    TiltRight = 1,
}

#[derive(Debug, Clone, PartialEq)]
struct GameInstruction {
    // (y, x)
    location: (u128, u128),
    tile: Tile,
}

#[derive(Debug, Clone, PartialEq)]
struct Screen {
    width: u128,
    height: u128,
    tiles: Vec<Vec<Tile>>,
}

pub fn main() {
    let positions = super::utils::get_list_of_numbers::<i128>();
    // println!("Positions {:?}", positions);
    // part1(&positions.clone());
    part2(&positions.clone());
}

fn part1(positions: &[i128]) {
    let map: RefCell<HashMap<(u128, u128), Tile>> = RefCell::new(HashMap::new());
    let outputs = RefCell::new(vec![]);
    run_program(
        positions.to_owned(),
        0,
        || super::utils::get_number_from_stdio::<i128>().unwrap(),
        |result: i128| {
            // println!("Output: {:?}", result);
            (*outputs.borrow_mut()).push(result);

            if (outputs.borrow()).len() >= 3 {
                let instruction = get_instruction(outputs.borrow().clone());
                // println!("Outputs: {:?}; {:?}", outputs.borrow(), instruction);

                (*map.borrow_mut()).insert(instruction.location, instruction.tile);

                outputs.replace(vec![]);
            }
        },
        |_state| false,
    );
    // Count answer for part 1
    let answer = map.borrow().values().filter(|v| **v == Tile::Block).count();
    println!("Answer: {:?}", answer);

    // Get the output and print it on console
}

// TODO: Wow, this game is hard...
fn part2(positions: &[i128]) {
    let map: RefCell<HashMap<(u128, u128), Tile>> = RefCell::new(HashMap::new());
    let outputs = RefCell::new(vec![]);
    let mut modified_positions = positions.to_owned();
    modified_positions[0] = 2; // To run game continuously
    let answer = RefCell::new(0);
    run_program(
        modified_positions,
        0,
        || {
            print_map(&*map.borrow());
            let term = Term::stdout();
            let input = term.read_key().unwrap();
            match input {
                console::Key::ArrowLeft => JoystickPosition::TiltLeft as i128,
                console::Key::ArrowRight => JoystickPosition::TiltRight as i128,
                _ => JoystickPosition::Neutral as i128,
            }
        },
        |result: i128| {
            // println!("Output: {:?}", result);
            (*outputs.borrow_mut()).push(result);

            if (outputs.borrow()).len() >= 3 {
                if outputs.borrow()[0] == -1 && outputs.borrow()[1] == 0 {
                    // Special case: output is a score
                    answer.replace(outputs.borrow()[2]);
                } else {
                    // Normal case, outputs are game instruction
                    let instruction = get_instruction(outputs.borrow().clone());
                    // println!("Outputs: {:?}; {:?}", outputs.borrow(), instruction);

                    (*map.borrow_mut()).insert(instruction.location, instruction.tile);
                }
                outputs.replace(vec![]);
            }
        },
        |_state| false,
    );
    print_map(&*map.borrow());

    clear_screen();
    println!("Answer part 2: {:?}", *answer.borrow());

    // Get the output and print it on console
}

fn get_instruction(raw_instruction: Vec<i128>) -> GameInstruction {
    GameInstruction {
        location: (raw_instruction[1] as u128, raw_instruction[0] as u128),
        tile: get_tile(raw_instruction[2]),
    }
}

fn get_tile(raw_tile: i128) -> Tile {
    match raw_tile {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::HorizontalPaddle,
        4 => Tile::Ball,
        _ => Tile::Empty,
    }
}

fn get_tile_as_str(tile: &Tile) -> String {
    let result = match tile {
        Tile::Empty => " ",
        Tile::Ball => "O",
        Tile::Block => "%",
        Tile::HorizontalPaddle => "=",
        Tile::Wall => "#",
    };
    result.to_string()
}

fn print_map(map: &HashMap<(u128, u128), Tile>) {
    let term = Term::stdout();
    term.clear_screen();
    for (location, tile) in map.iter() {
        term.move_cursor_to(location.1 as usize, location.0 as usize);
        term.write_str(&get_tile_as_str(tile));
    }
    thread::sleep(Duration::from_millis(400));
}

fn clear_screen() {
    let term = Term::stdout();
    term.clear_screen();
    thread::sleep(Duration::from_millis(400));
}
