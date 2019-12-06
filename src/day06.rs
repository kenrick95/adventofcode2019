use super::utils::get_string_from_stdio;
use std::collections::HashMap;

struct Orbit {
    object: String,
    children: Vec<Orbit>,
}

pub fn main() {
    let mut inputs: Vec<String> = vec![];
    loop {
        let input = get_string_from_stdio();
        if input.trim() == "" {
            break;
        }
        inputs.push(input);
    }

    // A)B --> A is parent of B
    // Can an Object orbits around 2 other Objects?
    let mut parent: HashMap<String, String> = HashMap::new();

    // Build tree

    let mut answer_pt1 = 0;
    // Count edges
}
