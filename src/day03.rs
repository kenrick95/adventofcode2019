#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: u32,
}

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}
const OFFSET: i32 = 5000;

fn transform_to_instruction(string: &str) -> Instruction {
    let first_letter = string.chars().nth(0).unwrap();
    let direction = match first_letter {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => {
            println!("first_letter not defined {:?}", first_letter);
            Direction::Up
        }
    };
    let num_str = &string[1..string.len()];
    let amount: u32 = num_str.parse().unwrap();

    return Instruction { direction, amount };
}

fn get_instructions() -> Vec<Instruction> {
    let raw_string = super::utils::get_string_from_stdio();
    return raw_string
        .trim()
        .split(",")
        .map(|val| transform_to_instruction(val))
        .collect();
}

fn get_state() -> Vec<Vec<bool>> {
    return vec![vec![false; (OFFSET * 2) as usize]; (OFFSET * 2) as usize];
}

// TODO: Need one more &mut args to mark the intersections
fn apply_instructions(state: &Vec<Vec<bool>>, instructions: Vec<Instruction>) -> Vec<Vec<bool>> {
    let mut coordinate = Coordinate {
        x: OFFSET,
        y: OFFSET,
    };
    for instruction in instructions {
        let dx: i32 = match instruction.direction {
            Direction::Left => -1 * instruction.amount as i32,
            Direction::Right => instruction.amount as i32,
            _ => 0,
        };
        let dy: i32 = match instruction.direction {
            Direction::Up => -1 * instruction.amount as i32,
            Direction::Down => instruction.amount as i32,
            _ => 0,
        };
        let next_coordinate = Coordinate {
            x: coordinate.x + dx,
            y: coordinate.y + dy,
        };

        // TODO: mark `state` from `coordinate` to `next_coordinate`, checking for whether `state` has been marked or not

        coordinate = next_coordinate;
    }

    return state.clone();
}

pub fn main() {
    let inst_1 = get_instructions();
    let inst_2 = get_instructions();

    println!("inst1: {:?}", inst_1);
    println!("inst2: {:?}", inst_2);

    let mut state = get_state();
    state = apply_instructions(&state, inst_1);
    state = apply_instructions(&state, inst_2);
}
