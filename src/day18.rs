use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Entrance,
    OpenPassage,
    Wall,
    Key,
    Door,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell {
    cell_type: CellType,

    /// Only have meaning for CellType::Key and CellType::Door
    cell_data: char,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct QueueState {
    /// (y, x)
    location: (usize, usize),

    step_count: usize,

    keys_found: Vec<char>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VisState {
    /// (y, x)
    location: (usize, usize),

    keys_found: Vec<char>,
}

pub fn main() {
    let inputs = super::utils::get_strings_from_file("./src/day18-real.log");

    let rows = inputs.len();
    let columns = inputs[0].len();
    let mut map: Vec<Vec<Cell>> = vec![
        vec![
            Cell {
                cell_type: CellType::Wall,
                cell_data: '#',
            };
            columns
        ];
        rows
    ];
    let mut keys_not_found: HashSet<char> = HashSet::new();
    // key_character -> (y, x)
    let mut keys_location: HashMap<char, (usize, usize)> = HashMap::new();

    // (y, x)
    let mut start_point: (usize, usize) = (0, 0);
    for (y, line) in inputs.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let cell_type = match ch {
                '@' => {
                    start_point = (y, x);
                    CellType::Entrance
                }
                '#' => CellType::Wall,
                '.' => CellType::OpenPassage,
                cha => {
                    if cha.is_ascii_alphabetic() {
                        if cha.is_uppercase() {
                            CellType::Door
                        } else {
                            keys_not_found.insert(cha);
                            keys_location.insert(cha, (y, x));
                            CellType::Key
                        }
                    } else {
                        CellType::Wall
                    }
                }
            };

            map[y][x] = Cell {
                cell_type,
                cell_data: ch.to_ascii_lowercase(),
            }
        }
    }
    println!("keys_location: {:?}", keys_location);

    // BFS here
    {
        let mut queue: VecDeque<QueueState> = VecDeque::new();
        let mut vis: HashMap<VisState, usize> = HashMap::new();
        let deltas: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        queue.push_back(QueueState {
            location: start_point,
            keys_found: vec![],
            step_count: 0,
        });

        while !queue.is_empty() {
            let current_state = queue.pop_front().unwrap();
            // println!("Q {:?}", current_state);

            // If alr reached goal, then break!
            if current_state.keys_found.len() == keys_location.len() {
                println!("Q {:?}", current_state);
                println!("Found! {}", current_state.step_count);
                break;
            }

            // Check state in `vis`, if going to state in `vis` is better than existing one, replace it
            for (dy, dx) in deltas.iter() {
                if current_state.location.0 as isize + dy >= 0
                    && current_state.location.0 as isize + dy < rows as isize
                    && current_state.location.1 as isize + dx >= 0
                    && current_state.location.1 as isize + dx < columns as isize
                {
                    let next_location = (
                        (current_state.location.0 as isize + dy) as usize,
                        (current_state.location.1 as isize + dx) as usize,
                    );
                    let next_cell_type = map[next_location.0][next_location.1].cell_type;
                    let next_cell_data = map[next_location.0][next_location.1].cell_data;
                    let mut current_state_keys = current_state.keys_found.clone();

                    // Can visit next_location?
                    // Conditions:
                    // 1. Not wall, AND
                    // 2. if Door, then MUST have key to it, AND
                    // 3. if vis[State] then
                    //        if current_state.step_count + 1 < vis[State] then VISIT
                    //    else VISIT
                    if next_cell_type == CellType::Wall {
                        continue;
                    }

                    let fulfil_condition_2 = if next_cell_type == CellType::Door {
                        current_state_keys.iter().any(|&key| key == next_cell_data)
                    } else {
                        true
                    };
                    if !fulfil_condition_2 {
                        continue;
                    }

                    // Pick key, if not already picked
                    if next_cell_type == CellType::Key
                        && current_state_keys.iter().all(|&key| key != next_cell_data)
                    {
                        current_state_keys.push(next_cell_data);
                        current_state_keys.sort();
                    }

                    let next_vis_state = VisState {
                        location: next_location,
                        keys_found: current_state_keys.clone(),
                    };

                    if vis.contains_key(&next_vis_state) {
                        let current_next_vis_state_step_count = vis.get(&next_vis_state).unwrap();
                        // Ever visited
                        if current_state.step_count + 1 < *current_next_vis_state_step_count {
                            vis.insert(next_vis_state, current_state.step_count + 1);
                            queue.push_back(QueueState {
                                location: next_location,
                                step_count: current_state.step_count + 1,
                                keys_found: current_state_keys,
                            });
                        }
                    } else {
                        // Never visited, mark visited
                        vis.insert(next_vis_state, current_state.step_count + 1);
                        queue.push_back(QueueState {
                            location: next_location,
                            step_count: current_state.step_count + 1,
                            keys_found: current_state_keys,
                        });
                    }
                }
            }
        }
    }
}
