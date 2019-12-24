use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Bug,
}

pub fn main() {
    let inputs = super::utils::get_strings_from_file("./src/day24-real.log");
    let mut map: Vec<Vec<Cell>> = vec![vec![Cell::Empty; 5]; 5];
    for (i, line) in inputs.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            map[i][j] = match ch {
                '#' => Cell::Bug,
                '.' => Cell::Empty,
                _ => continue,
            };
        }
    }
    part1(&map);
}

fn part1(map: &[Vec<Cell>]) {
    {
        let mut current_map = map.to_owned();
        let mut seen: HashSet<usize> = HashSet::new();
        let rows = current_map.len();
        let columns = current_map[0].len();
        let deltas: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let rating = get_rating(&current_map);
        println!("[{}] {:?}", 0, current_map);
        println!("[{}] {:?}", 0, rating);
        seen.insert(rating);
        for it in 1..=1000 {
            let mut new_map = current_map.clone();

            for (i, line) in current_map.iter().enumerate() {
                for (j, cell) in line.iter().enumerate() {
                    let new_cell = match cell {
                        Cell::Bug => {
                            let mut neighbour_bug_count = 0;
                            for (dy, dx) in &deltas {
                                if i as isize + dy >= 0
                                    && i as isize + dy < rows as isize
                                    && j as isize + dx >= 0
                                    && j as isize + dx < columns as isize
                                {
                                    let current_cell = current_map[(i as isize + dy) as usize]
                                        [(j as isize + dx) as usize];
                                    neighbour_bug_count +=
                                        if current_cell == Cell::Bug { 1 } else { 0 };
                                }
                            }
                            if neighbour_bug_count == 1 {
                                Cell::Bug
                            } else {
                                Cell::Empty
                            }
                        }
                        Cell::Empty => {
                            let mut neighbour_bug_count = 0;
                            for (dy, dx) in &deltas {
                                if i as isize + dy >= 0
                                    && i as isize + dy < rows as isize
                                    && j as isize + dx >= 0
                                    && j as isize + dx < columns as isize
                                {
                                    let current_cell = current_map[(i as isize + dy) as usize]
                                        [(j as isize + dx) as usize];
                                    neighbour_bug_count +=
                                        if current_cell == Cell::Bug { 1 } else { 0 };
                                }
                            }
                            if neighbour_bug_count == 1 || neighbour_bug_count == 2 {
                                Cell::Bug
                            } else {
                                Cell::Empty
                            }
                        }
                    };
                    new_map[i][j] = new_cell;
                }
            }
            let rating = get_rating(&new_map);
            println!("[{}] {:?}", it, new_map);
            println!("[{}] {:?}", it, rating);

            if seen.contains(&rating) {
                println!("[{}] Answer! {:?}", it, rating);
                break;
            }
            seen.insert(rating);

            current_map = new_map;
        }
    }
}

fn get_rating(map: &[Vec<Cell>]) -> usize {
    let mut rating = 0;

    let columns = map[0].len();
    for (i, line) in map.iter().enumerate() {
        for (j, &cell) in line.iter().enumerate() {
            if cell == Cell::Bug {
                let exponent = (i * columns + j) as u32;
                rating += usize::pow(2, exponent);
            }
        }
    }

    rating
}
