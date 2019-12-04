use std::collections::HashMap;
use std::collections::HashSet;

// Credits to https://www.reddit.com/r/adventofcode/comments/e5bz2w/2019_day_3_solutions/f9iz68s/ for inspirations

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

fn get_points(instructions: Vec<Instruction>) -> HashMap<(i32, i32), u32> {
    let mut x = 0;
    let mut y = 0;
    let mut length = 0;
    let mut points: HashMap<(i32, i32), u32> = HashMap::new();
    for instruction in instructions {
        let dx: i32 = match instruction.direction {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        };
        let dy: i32 = match instruction.direction {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        };
        for _i in 0..instruction.amount {
            x += dx;
            y += dy;
            length += 1;
            // println!("{:?}", (x,y));

            if !points.contains_key(&(x, y)) {
                points.insert((x, y), length);
            }
        }
    }

    return points;
}

/*
R8,U5,L5,D3
U7,R6,D4,L4
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83

*/
pub fn main() {
    let inst_1 = get_instructions();
    let inst_2 = get_instructions();

    // println!("inst1: {:?}", inst_1);
    // println!("inst2: {:?}", inst_2);

    let points_1 = get_points(inst_1);
    let points_2 = get_points(inst_2);
    // println!("points_1: {:?}", points_1);
    // println!("points_2: {:?}", points_2);

    // https://stackoverflow.com/questions/59156305/in-rust-how-do-i-create-a-hashset-from-the-keys-of-a-hashmap
    let points_1_coords: HashSet<(i32, i32)> = points_1.keys().copied().collect();
    let points_2_coords: HashSet<(i32, i32)> = points_2.keys().copied().collect();

    let points_intersections = points_1_coords.intersection(&points_2_coords);
    // println!("points_intersections: {:?}", points_intersections);
    let point_part1 = points_intersections
        .clone()
        .min_by_key(|v| v.0.abs() + v.1.abs())
        .unwrap();
    println!("Part 1: {:?}", point_part1.0.abs() + point_part1.1.abs());

    let point_part2 = points_intersections
        .clone()
        .min_by_key(|v| points_1.get(&v).unwrap() + points_2.get(&v).unwrap())
        .unwrap();
    // println!("{:?}", point_part2);
    println!(
        "Part 2: {:?}",
        points_1.get(&point_part2).unwrap() + points_2.get(&point_part2).unwrap()
    );
}
