#[derive(PartialEq, Debug)]
enum OpCode {
    Add,
    Multiply,
    Halt,
}

#[derive(Debug)]
struct State {
    positions: Vec<u32>,
    program_counter: u32,
}

fn reducer(state: &State, action: OpCode) -> State {
    let mut new_state = State {
        positions: state.positions.clone(),
        program_counter: state.program_counter + 4,
    };

    let program_counter = state.program_counter as usize;
    let pos_a = state.positions[program_counter + 1];
    let pos_b = state.positions[program_counter + 2];
    let pos_res = state.positions[program_counter + 3];
    match action {
        OpCode::Add => {
            let result = state.positions[pos_a as usize] + state.positions[pos_b as usize];
            new_state.positions[pos_res as usize] = result;
        }
        OpCode::Multiply => {
            let result = state.positions[pos_a as usize] * state.positions[pos_b as usize];
            new_state.positions[pos_res as usize] = result;
        }
        OpCode::Halt => {}
    }
    return new_state;
}

fn get_action(state: &State) -> OpCode {
    let program_counter = state.program_counter;
    let raw_opcode = state.positions[program_counter as usize];
    if raw_opcode == 1 {
        return OpCode::Add;
    } else if raw_opcode == 2 {
        return OpCode::Multiply;
    }
    return OpCode::Halt;
}

fn run_program(positions: Vec<u32>) -> State {
    let program_counter = 0;
    let mut state = State {
        positions,
        program_counter,
    };

    let mut iteration = 0;
    while iteration < 1000 {
        let action = get_action(&state);
        if action == OpCode::Halt {
            break;
        }
        // println!("iteration1 {:?}: {:?} {:?}", iteration, state, action);
        let next_state = reducer(&state, action);
        state = next_state;
        // println!("iteration2 {:?}: {:?}", iteration, state);
        if state.program_counter as usize >= state.positions.len() {
            break;
        }
        iteration += 1;
    }
    // println!("Final {:?}", state);
    return state;
}

fn run_program_with_tweak(positions: &Vec<u32>, a: u32, b: u32) -> State {
    let mut temp = positions.clone();
    temp[1] = a;
    temp[2] = b;
    return run_program(temp);
}

pub fn main() {
    let positions: Vec<u32> = super::utils::get_string_from_stdio()
        .trim()
        .split(",")
        .map(|val| val.parse().unwrap())
        .collect();
    println!("Positions {:?}", positions);

    println!(
        "Part 1 answer {:?}",
        run_program_with_tweak(&positions, 12, 2).positions[0]
    );

    {
        let mut found = false;
        for a in 0..99 {
            for b in 0..99 {
                if run_program_with_tweak(&positions, a, b).positions[0] == 19690720 {
                    found = true;
                    println!("Part 2 answer {:?}", a * 100 + b);
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            println!("Part 2 not found!?");
        }
    }
}
