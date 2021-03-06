use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Wall,
    None,
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
    location: (u8, u8),

    step_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VisState {
    /// (y, x)
    location: (u8, u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct QueueState2 {
    /// (y, x)
    location: (u8, u8),
    level: usize,
    step_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VisState2 {
    /// (y, x)
    location: (u8, u8),
    level: usize,
}

pub fn main() {
    // "./src/day20-real.log";
    let inputs = super::utils::get_strings_from_file_no_trim("./src/day20-real.log");
    let input_chars: Vec<Vec<char>> = inputs.iter().map(|line| line.chars().collect()).collect();
    let rows = inputs.len();
    let columns = inputs[0].len();
    println!("rows {}", rows);
    println!("columns {}", columns);
    let mut map: Vec<Vec<Cell>> = vec![
        vec![
            Cell {
                cell_type: CellType::None,
                cell_data: " ".to_string(),
            };
            columns
        ];
        rows
    ];
    // "AB" -> ((y1, x1), (y2, x2))
    let mut portals_by_label: HashMap<String, ((u8, u8), (u8, u8))> = HashMap::new();
    // (y1, x1) -> (y2, x2); also in reverse
    let mut portals_by_location: HashMap<(u8, u8), (u8, u8)> = HashMap::new();
    // (y, x) -> is_inner
    let mut portal_is_inner: HashMap<(u8, u8), bool> = HashMap::new();

    // (y, x)
    let mut start_point: (u8, u8) = (0, 0);
    let mut end_point: (u8, u8) = (1, 1);
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
                _ => CellType::None,
            };

            if cell_data == "AA" {
                start_point = (y as u8, x as u8);
                cell_type = CellType::OpenPassage;
            } else if cell_data == "ZZ" {
                end_point = (y as u8, x as u8);
                cell_type = CellType::OpenPassage;
            }
            // println!("a {} {}; {}; {:?}; {:?}", y, x, ch, cell_type, cell_data);

            if cell_type == CellType::Portal {
                let is_inner_portal = if x <= 2 || x >= columns - 3 || y <= 2 || y >= rows - 3 {
                    false
                } else {
                    true
                };

                portal_is_inner.insert((y as u8, x as u8), is_inner_portal);

                let current_portal_value = portals_by_label
                    .get(&cell_data)
                    .unwrap_or(&((0, 0), (0, 0)));
                let first_location = current_portal_value.0;
                if first_location.0 == 0 && first_location.1 == 0 {
                    // Never inserted
                    portals_by_label.insert(cell_data.clone(), ((y as u8, x as u8), (0, 0)));
                } else {
                    // Once inserted
                    portals_by_label
                        .insert(cell_data.clone(), (first_location, (y as u8, x as u8)));
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

    // part1(start_point, end_point, &portals_by_location, &map);
    part2(
        start_point,
        end_point,
        &portals_by_location,
        &portal_is_inner,
        &map,
    );
}

fn part1(
    start_point: (u8, u8),
    end_point: (u8, u8),
    portals_by_location: &HashMap<(u8, u8), (u8, u8)>,
    map: &Vec<Vec<Cell>>,
) {
    let rows = map.len();
    let columns = map[0].len();
    let mut queue: VecDeque<QueueState> = VecDeque::new();
    let mut vis: HashMap<VisState, usize> = HashMap::new();
    let deltas: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    vis.insert(
        VisState {
            location: start_point,
        },
        0,
    );
    queue.push_back(QueueState {
        location: start_point,
        step_count: 0,
    });
    while !queue.is_empty() {
        let current_state = queue.pop_front().unwrap();

        // If alr reached goal, then break!
        if current_state.location == end_point {
            println!("Q {:?}", current_state);
            println!("Found! {}", current_state.step_count);
            break;
        }
        let current_location = current_state.location;
        let current_location_cell =
            map[current_location.0 as usize][current_location.1 as usize].clone();
        // println!("C {:?}", current_location_cell);

        let mut next_locations: Vec<(u8, u8)> = vec![];

        for (dy, dx) in deltas.iter() {
            if current_location.0 as isize + dy >= 0
                && current_location.0 as isize + dy < rows as isize
                && current_location.1 as isize + dx >= 0
                && current_location.1 as isize + dx < columns as isize
            {
                let next_location = (
                    (current_location.0 as isize + dy) as u8,
                    (current_location.1 as isize + dx) as u8,
                );
                let next_cell_type =
                    map[next_location.0 as usize][next_location.1 as usize].cell_type;
                if next_cell_type == CellType::OpenPassage || next_cell_type == CellType::Portal {
                    next_locations.push(next_location);
                } else if next_cell_type == CellType::None
                    && current_location_cell.cell_type == CellType::Portal
                {
                    // next_location should be a portal
                    let next_location_real = *portals_by_location.get(&current_location).unwrap();
                    next_locations.push(next_location_real);
                }
            }
        }

        // println!("S {:?}", next_locations);

        for next_location in next_locations {
            // if vis[State] then
            //        if current_state.step_count + 1 < vis[State] then VISIT
            //    else VISIT

            let next_vis_state = VisState {
                location: next_location,
            };

            if vis.contains_key(&next_vis_state) {
                let current_next_vis_state_step_count = vis.get(&next_vis_state).unwrap();
                // Ever visited
                if current_state.step_count + 1 < *current_next_vis_state_step_count {
                    vis.insert(next_vis_state, current_state.step_count + 1);
                    queue.push_back(QueueState {
                        location: next_location,
                        step_count: current_state.step_count + 1,
                    });
                }
            } else {
                // Never visited, mark visited
                vis.insert(next_vis_state, current_state.step_count + 1);
                queue.push_back(QueueState {
                    location: next_location,
                    step_count: current_state.step_count + 1,
                });
            }
        }
    }
}

fn part2(
    start_point: (u8, u8),
    end_point: (u8, u8),
    portals_by_location: &HashMap<(u8, u8), (u8, u8)>,
    portal_is_inner: &HashMap<(u8, u8), bool>,
    map: &Vec<Vec<Cell>>,
) {

    // TODO: Make it a weighted graph construction, since in every level, the "BFS" are going to be the same.
    // The current naive way is taking too much RAM
    let rows = map.len();
    let columns = map[0].len();
    let mut queue: VecDeque<QueueState2> = VecDeque::new();
    let mut vis: HashMap<VisState2, usize> = HashMap::new();
    let deltas: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    vis.insert(
        VisState2 {
            location: start_point,
            level: 0,
        },
        0,
    );
    queue.push_back(QueueState2 {
        location: start_point,
        step_count: 0,
        level: 0,
    });
    while !queue.is_empty() {
        let current_state = queue.pop_front().unwrap();

        // If alr reached goal, then break!
        if current_state.location == end_point && current_state.level == 0 {
            println!("Q {:?}", current_state);
            println!("Found! {}", current_state.step_count);
            break;
        }
        let current_location = current_state.location;
        let current_location_cell =
            map[current_location.0 as usize][current_location.1 as usize].clone();
        // println!("C {:?}", current_location_cell);

        // (y, x, level)
        let mut next_locations: Vec<(u8, u8, usize)> = vec![];

        for (dy, dx) in deltas.iter() {
            if current_location.0 as isize + dy >= 0
                && current_location.0 as isize + dy < rows as isize
                && current_location.1 as isize + dx >= 0
                && current_location.1 as isize + dx < columns as isize
            {
                let next_location = (
                    (current_location.0 as isize + dy) as u8,
                    (current_location.1 as isize + dx) as u8,
                    current_state.level,
                );
                let next_cell_type =
                    map[next_location.0 as usize][next_location.1 as usize].cell_type;
                if next_cell_type == CellType::OpenPassage || next_cell_type == CellType::Portal {
                    next_locations.push(next_location);
                } else if next_cell_type == CellType::None
                    && current_location_cell.cell_type == CellType::Portal
                {
                    let is_inner = *portal_is_inner.get(&current_location).unwrap();
                    if !is_inner && current_state.level == 0 {
                        // Cannot go to negative level
                        continue;
                    }
                    let next_level = if is_inner {
                        current_state.level + 1
                    } else {
                        current_state.level - 1
                    };
                    // next_location should be a portal
                    let next_location_real = *portals_by_location.get(&current_location).unwrap();
                    next_locations.push((next_location_real.0, next_location_real.1, next_level));
                }
            }
        }

        // println!("S {:?}", next_locations);

        for (next_y, next_x, next_level) in next_locations {
            // if vis[State] then
            //        if current_state.step_count + 1 < vis[State] then VISIT
            //    else VISIT
            let next_location = (next_y, next_x);

            let next_vis_state = VisState2 {
                location: next_location,
                level: next_level,
            };

            if vis.contains_key(&next_vis_state) {
                let current_next_vis_state_step_count = vis.get(&next_vis_state).unwrap();
                // Ever visited
                if current_state.step_count + 1 < *current_next_vis_state_step_count {
                    vis.insert(next_vis_state, current_state.step_count + 1);
                    queue.push_back(QueueState2 {
                        location: next_location,
                        level: next_level,
                        step_count: current_state.step_count + 1,
                    });
                }
            } else {
                // Never visited, mark visited
                vis.insert(next_vis_state, current_state.step_count + 1);
                queue.push_back(QueueState2 {
                    location: next_location,
                    level: next_level,
                    step_count: current_state.step_count + 1,
                });
            }
        }
    }
}
