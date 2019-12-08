use std::collections::HashMap;

pub fn main() {
    let input: Vec<u8> = super::utils::get_string_from_stdio()
        .trim()
        .to_string()
        .chars()
        .map(|val| val.to_string().parse().unwrap())
        .collect();

    let width: usize = 25;
    let height: usize = 6;
    let area = width * height;

    let num_layers = input.len() / area;
    let mut layers_flat: Vec<Vec<u8>> = vec![vec![0; area]; num_layers];
    let mut digits_per_layer: Vec<HashMap<u8, usize>> = vec![HashMap::new(); num_layers];
    for l in 0..num_layers {
        for d in 0..=9 {
            digits_per_layer[l].insert(d, 0);
        }
    }
    let mut i = 0;
    for l in 0..num_layers {
        for j in 0..area {
            layers_flat[l][j] = input[i];
            let counter = digits_per_layer[l].entry(input[i]).or_insert(0);
            *counter += 1;

            i += 1;
        }
        println!("layer {:}: {:?}", l, layers_flat[l]);
    }

    let mut layer_with_fewest_zero = 0;
    let mut fewest_zero_count = digits_per_layer[0].get(&0).unwrap();
    for l in 0..num_layers {
        let zero_count = digits_per_layer[l].get(&0).unwrap();
        if zero_count < fewest_zero_count {
            fewest_zero_count = zero_count;
            layer_with_fewest_zero = l;
        }
    }

    let answer_pt1 = digits_per_layer[layer_with_fewest_zero].get(&1).unwrap()
        * digits_per_layer[layer_with_fewest_zero].get(&2).unwrap();

    println!("Part 1: {:?}", answer_pt1);

    let mut final_layer = vec![0; area];
    // 0: black
    // 1: white
    // 2: transparent
    for j in 0..area {
        final_layer[j] = 2;
        for l in 0..num_layers {
            if layers_flat[l][j] != 2 {
                final_layer[j] = layers_flat[l][j];
                break;
            }
        }
    }
    println!("final_layer {:?}", final_layer);

    // Pretty print
    for y in 0..height {
        for x in 0..width {
            print!(
                "{:}",
                if final_layer[x + y * width] == 1 {
                    "1"
                } else {
                    " "
                }
            );
        }
        println!("");
    }
}
