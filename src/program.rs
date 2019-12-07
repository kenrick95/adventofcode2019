#[derive(PartialEq, Debug, Copy, Clone)]
pub enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ParameterMode {
    /**
     * "position" as reference to address
     */
    Position,
    /**
     * "position" as value
     * Parameters that an instruction writes to will never be in immediate mode.
     */
    Immediate,
}

#[derive(PartialEq, Debug)]
pub struct Operation {
    opcode: OpCode,
    parameter_count: u32,
    modes: Vec<ParameterMode>,
}

#[derive(Debug)]
pub struct State {
    pub positions: Vec<i32>,
    pub program_counter: u32,
}

pub fn get_value(state: &State, index: u32, mode: ParameterMode) -> i32 {
    let raw_op = state.positions[index as usize];
    if mode == ParameterMode::Immediate {
        return raw_op;
    } else {
        return state.positions[raw_op as usize];
    }
}

pub fn reducer<I, O>(state: &State, operation: &Operation, get_input: I, write_output: O) -> State
where
    I: Fn() -> i32,
    O: Fn(i32),
{
    let mut new_state = State {
        positions: state.positions.clone(),
        program_counter: state.program_counter + 1 + operation.parameter_count,
    };

    let program_counter = state.program_counter;
    match operation.opcode {
        OpCode::Add => {
            let param_a = get_value(state, program_counter + 1, operation.modes[0]);
            let param_b = get_value(state, program_counter + 2, operation.modes[1]);
            let result = param_a + param_b;
            let pos_res = state.positions[(program_counter + 3) as usize];
            new_state.positions[pos_res as usize] = result;
        }
        OpCode::Multiply => {
            let param_a = get_value(state, program_counter + 1, operation.modes[0]);
            let param_b = get_value(state, program_counter + 2, operation.modes[1]);
            let result = param_a * param_b;
            let pos_res = state.positions[(program_counter + 3) as usize];
            new_state.positions[pos_res as usize] = result;
        }
        OpCode::Input => {
            // Ask for input
            let result: i32 = get_input();
            let pos_res = state.positions[(program_counter + 1) as usize];
            new_state.positions[pos_res as usize] = result;
        }
        OpCode::Output => {
            let param = get_value(state, program_counter + 1, operation.modes[0]);
            write_output(param);
            // println!("Output: {:?}", param);
        }
        OpCode::JumpIfTrue => {
            let param_a = get_value(state, program_counter + 1, operation.modes[0]);
            let param_b = get_value(state, program_counter + 2, operation.modes[1]);
            if param_a != 0 {
                new_state.program_counter = param_b as u32;
            }
        }
        OpCode::JumpIfFalse => {
            let param_a = get_value(state, program_counter + 1, operation.modes[0]);
            let param_b = get_value(state, program_counter + 2, operation.modes[1]);
            if param_a == 0 {
                new_state.program_counter = param_b as u32;
            }
        }
        OpCode::LessThan => {
            let param_a = get_value(state, program_counter + 1, operation.modes[0]);
            let param_b = get_value(state, program_counter + 2, operation.modes[1]);
            let pos_res = state.positions[(program_counter + 3) as usize];
            new_state.positions[pos_res as usize] = if param_a < param_b { 1 } else { 0 };
        }
        OpCode::Equals => {
            let param_a = get_value(state, program_counter + 1, operation.modes[0]);
            let param_b = get_value(state, program_counter + 2, operation.modes[1]);
            let pos_res = state.positions[(program_counter + 3) as usize];
            new_state.positions[pos_res as usize] = if param_a == param_b { 1 } else { 0 };
        }
        OpCode::Halt => {}
    }
    return new_state;
}

pub fn get_operation(state: &State) -> Operation {
    let program_counter = state.program_counter;
    let raw_opcode = state.positions[program_counter as usize] as u32;
    let opcode_number = raw_opcode % 100;
    let mut modes_number: u32 = raw_opcode / 100;
    let mut operation = Operation {
        opcode: OpCode::Halt,
        parameter_count: 0,
        modes: vec![],
    };
    match opcode_number {
        1 => {
            operation.opcode = OpCode::Add;
            operation.parameter_count = 3;
        }
        2 => {
            operation.opcode = OpCode::Multiply;
            operation.parameter_count = 3;
        }
        3 => {
            operation.opcode = OpCode::Input;
            operation.parameter_count = 1;
        }
        4 => {
            operation.opcode = OpCode::Output;
            operation.parameter_count = 1;
        }
        5 => {
            operation.opcode = OpCode::JumpIfTrue;
            operation.parameter_count = 2;
        }
        6 => {
            operation.opcode = OpCode::JumpIfFalse;
            operation.parameter_count = 2;
        }
        7 => {
            operation.opcode = OpCode::LessThan;
            operation.parameter_count = 3;
        }
        8 => {
            operation.opcode = OpCode::Equals;
            operation.parameter_count = 3;
        }
        _ => {}
    }
    // Convert `modes_number` to modes
    for _i in 0..operation.parameter_count {
        let res = modes_number % 10;
        modes_number = modes_number / 10;
        operation.modes.push(if res == 1 {
            ParameterMode::Immediate
        } else {
            ParameterMode::Position
        });
    }

    return operation;
}

pub fn run_program<I, O>(raw_positions: Vec<i32>, get_input: I, write_output: O) -> State
where
    I: Fn() -> i32,
    O: Fn(i32),
{
    let program_counter = 0;
    let mut positions = vec![0; 10000];
    for (i, post) in raw_positions.iter().enumerate() {
        positions[i] = *post;
    }

    let mut state = State {
        positions,
        program_counter,
    };

    let mut iteration = 0;
    while iteration < 1000 {
        let operation = get_operation(&state);
        // println!("it {:?}, {:?}", iteration, operation);
        if operation.opcode == OpCode::Halt {
            break;
        }
        // println!("iteration1 {:?}: {:?} {:?}", iteration, state, operation);
        let next_state = reducer(&state, &operation, &get_input, &write_output);
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
