use super::program::*;
use console::Term;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn main() {
    let positions = super::utils::get_list_of_numbers_from_file::<i128>("./src/day19-real.log");
    part1(&positions);
    // part2(&positions);
}

fn get_result_on_location(raw_positions: &[i128], location: (i128, i128)) -> i128 {
    let y = location.0;
    let x = location.1;
    let input_count: RefCell<usize> = RefCell::new(0);
    let final_result: RefCell<i128> = RefCell::new(0);
    run_program(
        raw_positions.to_owned(),
        0,
        0,
        || {
            // println!("input {} {}", x, y);
            if *input_count.borrow() == 0 {
                *input_count.borrow_mut() += 1;
                x
            } else {
                y
            }
        },
        |result: i128| {
            final_result.replace(result);
        },
        |_state| {
            if *input_count.borrow() == 2 {
                true
            } else {
                false
            }
        },
    );
    let returned_result = *final_result.borrow();
    returned_result.clone()
}

fn part1(positions: &[i128]) {
    let mut map: HashMap<(usize, usize), bool> = HashMap::new();
    let mut answer = 0;
    for y in 0..125 {
        for x in 0..125 {
            let result = get_result_on_location(positions, (y, x));
            if x < 50 && y < 50 {
                answer += result;
            }
            map.insert(
                (y as usize, x as usize),
                if result == 1 { true } else { false },
            );
        }
    }

    print_map(&map);
    println!("Answer part 1: {:?}", answer);
}

fn print_map(map: &HashMap<(usize, usize), bool>) {
    let term = Term::stdout();
    let mut max_x = 0;
    let mut max_y = 0;
    term.clear_screen();
    for (location, &cell) in map.iter() {
        term.move_cursor_to(location.1, location.0);
        max_x = std::cmp::max(max_x, location.1);
        max_y = std::cmp::max(max_y, location.0);
        term.write_str(if cell { "#" } else { "." });
    }
    term.move_cursor_to(max_x, max_y);
    term.write_line("");
    term.write_line(&max_y.to_string());
    term.write_line(&max_x.to_string());
    // let input = term.read_key().unwrap();
}
