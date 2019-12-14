use super::program::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
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
    println!("Positions {:?}", positions);
    part1(&positions.clone());
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
