use super::utils::get_string_from_stdio;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn main() {
    let mut inputs: Vec<String> = vec![];
    loop {
        let input = get_string_from_stdio();
        if input.trim() == "" {
            break;
        }
        inputs.push(String::from(input.trim()));
    }
    // let num_edges = inputs.len();
    let mut nodes: HashSet<String> = HashSet::new();

    // A)B --> A is parent of B
    // Can an Object orbits around 2 other Objects?
    // parent: key = self; value = parent
    let mut parent_map: HashMap<String, String> = HashMap::new();
    for input in inputs {
        let parts: Vec<&str> = input.split(")").collect();
        let obj_a = String::from(parts[0]);
        let obj_b = String::from(parts[1]);
        nodes.insert(obj_a.clone());
        nodes.insert(obj_b.clone());
        parent_map.insert(obj_b, obj_a);
    }

    let mut total_depths = 0;
    for node in nodes {
        // println!("node: {}; depth {}", node, depth);
        total_depths += get_depth(&parent_map, node);
    }

    // println!("num_edges: {}", num_edges);
    // println!("total_depths: {}", total_depths);
    let answer_pt1 = total_depths;
    println!("Part 1: {}", answer_pt1);

    // Find the ancestors of `YOU`
    let ancestors_you = get_ancestors(&parent_map, "YOU".to_string());
    // Find the ancestors of `SAN`
    let ancestors_san = get_ancestors(&parent_map, "SAN".to_string());

    // println!("ancestors_you: {:?}", ancestors_you);
    // println!("ancestors_san: {:?}", ancestors_san);

    let lca_you_san = get_lca(&ancestors_you, &ancestors_san);

    let depth_you = get_depth(&parent_map, "YOU".to_string());
    let depth_san = get_depth(&parent_map, "SAN".to_string());
    let depth_lca = get_depth(&parent_map, lca_you_san);
    // println!("depth_you: {}", depth_you);
    // println!("depth_san: {}", depth_san);
    // println!("depth_lca: {}", depth_lca);
    let answer_pt2 = depth_you + depth_san - depth_lca - depth_lca - 2;

    // Part 2 answer = distance from YOU to LCA + distance from SAN to LCA - 2
    println!("Part 2 {}", answer_pt2);
}

fn get_depth(parent_map: &HashMap<String, String>, node: String) -> usize {
    let mut depth = 0;
    let mut current_node = node.clone();
    while parent_map.contains_key(&current_node) {
        let parent_node = parent_map.get(&current_node).unwrap().clone();
        current_node = parent_node;
        depth += 1;
    }
    return depth;
}

fn get_ancestors(parent_map: &HashMap<String, String>, node: String) -> Vec<String> {
    let mut result = vec![];
    let mut current_node = node;
    while parent_map.contains_key(&current_node) {
        let parent_node = parent_map.get(&current_node).unwrap();
        current_node = parent_node.clone();
        result.push(parent_node.clone());
    }
    return result;
}

fn get_lca(ancestors_a_in: &Vec<String>, ancestors_b_in: &Vec<String>) -> String {
    let mut ancestors_a = ancestors_a_in.clone();
    let mut ancestors_b = ancestors_b_in.clone();
    ancestors_a.reverse();
    ancestors_b.reverse();
    let min_len = usize::min(ancestors_a.len(), ancestors_b.len());

    let mut result = ancestors_a[0].clone();
    for i in 0..min_len {
        if ancestors_a[i] == ancestors_b[i] {
            result = ancestors_a[i].clone();
        } else {
            break;
        }
    }

    return result;
}
