use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Wall,
    OpenPassage,
    Portal,
}

#[derive(Debug, Clone, PartialEq)]
struct Cell {
    cell_type: CellType,

    /// Only have meaning for CellType::Portal
    cell_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct QueueState {
    /// (y, x)
    location: (usize, usize),

    step_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VisState {
    /// (y, x)
    location: (usize, usize),
}

pub fn main() {
    // "./src/day20-real.log";
    let inputs = super::utils::get_strings_from_file_no_trim("./src/day20-test1.log");
    let input_chars: Vec<Vec<char>> = inputs.iter().map(|line| line.chars().collect()).collect();
    let rows = inputs.len();
    let columns = inputs[0].len();
    println!("rows {}", rows);
    println!("columns {}", columns);
    let mut map: Vec<Vec<Cell>> = vec![
        vec![
            Cell {
                cell_type: CellType::Wall,
                cell_data: "#".to_string(),
            };
            columns
        ];
        rows
    ];
    // "AB" -> ((y1, x1), (y2, x2))
    let mut portals_by_label: HashMap<String, ((usize, usize), (usize, usize))> = HashMap::new();
    // (y1, x1) -> (y2, x2); also in reverse
    let mut portals_by_location: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    // (y, x)
    let mut start_point: (usize, usize) = (0, 0);
    let mut end_point: (usize, usize) = (1, 1);
    for (y, line) in input_chars.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            let mut cell_data = ch.to_ascii_uppercase().to_string();
            let mut cell_type = match ch {
                '#' => CellType::Wall,
                '.' => {
                    // if at edge and next to a character, then it should be a portal
                    if x > 1 && line[x - 1].is_ascii_alphanumeric() {
                        cell_data = "".to_string();
                        cell_data.push(line[x - 2]);
                        cell_data.push(line[x - 1]);
                        CellType::Portal
                    } else if x < columns - 2 && line[x + 1].is_ascii_alphanumeric() {
                        cell_data = "".to_string();
                        cell_data.push(line[x + 1]);
                        cell_data.push(line[x + 2]);
                        CellType::Portal
                    } else if y > 1 && input_chars[y - 1][x].is_ascii_alphanumeric() {
                        cell_data = "".to_string();
                        cell_data.push(input_chars[y - 2][x]);
                        cell_data.push(input_chars[y - 1][x]);
                        CellType::Portal
                    } else if y < rows - 2 && input_chars[y + 1][x].is_ascii_alphanumeric() {
                        cell_data = "".to_string();
                        cell_data.push(input_chars[y + 1][x]);
                        cell_data.push(input_chars[y + 2][x]);
                        CellType::Portal
                    } else {
                        CellType::OpenPassage
                    }
                }
                _ => CellType::Wall,
            };

            if cell_data == "AA" {
                start_point = (y, x);
                cell_type = CellType::OpenPassage;
            } else if cell_data == "ZZ" {
                end_point = (y, x);
                cell_type = CellType::OpenPassage;
            }
            // println!("a {} {}; {}; {:?}; {:?}", y, x, ch, cell_type, cell_data);

            if cell_type == CellType::Portal {
                let current_portal_value = portals_by_label
                    .get(&cell_data)
                    .unwrap_or(&((0, 0), (0, 0)));
                let first_location = current_portal_value.0;
                if first_location.0 == 0 && first_location.1 == 0 {
                    // Never inserted
                    portals_by_label.insert(cell_data.clone(), ((y, x), (0, 0)));
                } else {
                    // Once inserted
                    portals_by_label.insert(cell_data.clone(), (first_location, (y, x)));
                }
            }

            map[y][x] = Cell {
                cell_type,
                cell_data,
            }
        }
    }

    // Generate portals_by_location
    for (first_location, second_location) in portals_by_label.values() {
        portals_by_location.insert(first_location.clone(), second_location.clone());
        portals_by_location.insert(second_location.clone(), first_location.clone());
    }

    println!("start_point {:?}", start_point);
    println!("end_point {:?}", end_point);
    println!("portals_by_location {:?}", portals_by_location);

    // BFS here
    // {
    //     let mut queue: VecDeque<QueueState> = VecDeque::new();
    //     let mut vis: HashMap<VisState, usize> = HashMap::new();
    //     let deltas: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    //     queue.push_back(QueueState {
    //         locations: start_points,
    //         keys_found: vec![],
    //         step_count: 0,
    //     });
    //     while !queue.is_empty() {
    //         let current_state = queue.pop_front().unwrap();

    //         // If alr reached goal, then break!
    //         if current_state.keys_found.len() == keys_location.len() {
    //             println!("Q {:?}", current_state);
    //             println!("Found! {}", current_state.step_count);
    //             break;
    //         }

    //         for (i, current_location) in current_state.locations.iter().enumerate() {
    //             // Check state in `vis`, if going to state in `vis` is better than existing one, replace it
    //             for (dy, dx) in deltas.iter() {
    //                 if current_location.0 as isize + dy >= 0
    //                     && current_location.0 as isize + dy < rows as isize
    //                     && current_location.1 as isize + dx >= 0
    //                     && current_location.1 as isize + dx < columns as isize
    //                 {
    //                     let next_location = (
    //                         (current_location.0 as isize + dy) as usize,
    //                         (current_location.1 as isize + dx) as usize,
    //                     );
    //                     let next_cell_type = map[next_location.0][next_location.1].cell_type;
    //                     let next_cell_data = map[next_location.0][next_location.1].cell_data;
    //                     let mut current_state_keys = current_state.keys_found.clone();

    //                     // Can visit next_location?
    //                     // Conditions:
    //                     // 1. Not wall, AND
    //                     // 2. if Door, then MUST have key to it, AND
    //                     // 3. if vis[State] then
    //                     //        if current_state.step_count + 1 < vis[State] then VISIT
    //                     //    else VISIT
    //                     if next_cell_type == CellType::Wall {
    //                         continue;
    //                     }

    //                     let fulfil_condition_2 = if next_cell_type == CellType::Door {
    //                         current_state_keys.iter().any(|&key| key == next_cell_data)
    //                     } else {
    //                         true
    //                     };
    //                     if !fulfil_condition_2 {
    //                         continue;
    //                     }

    //                     // Pick key, if not already picked
    //                     if next_cell_type == CellType::Key
    //                         && current_state_keys.iter().all(|&key| key != next_cell_data)
    //                     {
    //                         current_state_keys.push(next_cell_data);
    //                         current_state_keys.sort();
    //                     }

    //                     let mut next_locations = current_state.locations.clone();
    //                     next_locations[i] = next_location;

    //                     let next_vis_state = VisState {
    //                         locations: next_locations.clone(),
    //                         keys_found: current_state_keys.clone(),
    //                     };

    //                     if vis.contains_key(&next_vis_state) {
    //                         let current_next_vis_state_step_count =
    //                             vis.get(&next_vis_state).unwrap();
    //                         // Ever visited
    //                         if current_state.step_count + 1 < *current_next_vis_state_step_count {
    //                             vis.insert(next_vis_state, current_state.step_count + 1);
    //                             queue.push_back(QueueState {
    //                                 locations: next_locations,
    //                                 step_count: current_state.step_count + 1,
    //                                 keys_found: current_state_keys,
    //                             });
    //                         }
    //                     } else {
    //                         // Never visited, mark visited
    //                         vis.insert(next_vis_state, current_state.step_count + 1);
    //                         queue.push_back(QueueState {
    //                             locations: next_locations,
    //                             step_count: current_state.step_count + 1,
    //                             keys_found: current_state_keys,
    //                         });
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}
