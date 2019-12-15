use super::utils::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
struct Reaction {
    product: (usize, String),
    reactants: Vec<(usize, String)>,
}

pub fn main() {
    let mut reactions: Vec<Reaction> = vec![];
    /*
     * K {String} substance id
     * V {usize} index in `reactions`
     *
     * This means that reaction[V] has product = K
     */
    let mut reaction_by_product: HashMap<String, usize> = HashMap::new();
    // 10 ORE, 3 A, 4 B => 1 FUEL
    let re = Regex::new(r"(\d+) ([[:alpha:]]+)").unwrap();
    loop {
        let input = get_string_from_stdio().trim().to_string();
        if input == "" {
            break;
        }
        let splitted: Vec<&str> = input.split("=>").collect();
        let lhs = splitted[0].trim();
        let rhs = splitted[1].trim();

        let captures_lhs_mult = re.captures_iter(lhs);
        let captures_rhs = re.captures(rhs).unwrap();
        let mut groups: Vec<(usize, String)> = vec![];
        let mut i;
        for captures_lhs in captures_lhs_mult {
            // println!("Captures {:?}", captures_lhs);
            i = 1;
            while i < captures_lhs.len() {
                groups.push((
                    captures_lhs.get(i).unwrap().as_str().parse().unwrap(),
                    captures_lhs.get(i + 1).unwrap().as_str().to_string(),
                ));
                i += 2;
            }
        }

        // println!("Captures {:?}", captures_rhs);
        i = 1;
        while i < captures_rhs.len() {
            groups.push((
                captures_rhs.get(i).unwrap().as_str().parse().unwrap(),
                captures_rhs.get(i + 1).unwrap().as_str().to_string(),
            ));
            i += 2;
        }
        println!("groups {:?}", groups);

        let product = groups.pop().unwrap();
        let reactants = groups;

        reaction_by_product.insert(product.1.clone(), reactions.len());
        reactions.push(Reaction { reactants, product });
    }
    println!("reactions {:?}", reactions);

    let reaction_index = *reaction_by_product.get("FUEL").unwrap();
    let mut reaction = reactions[reaction_index].clone();

    // To generate 1 FUEL, how many ORE is needed?
    // Example
    // 1 ORE => 4 A
    // 1 A => 1 B
    // 3 A, 1 B => 1 FUEL
    // i.e. need only 1 ORE

    // Start from the reaction that generate "FUEL"
    // do stoichiometry magic by "eliminating" one reactant at a time until we're stuck with "ORE"
    // TODO: This is proper stoichiometry, but this problem cannot be solved that way
    // step = 1, eliminating A
    // 1 ORE      => 4 A     | x3
    // 3 A, 1 B   => 1 FUEL  | x4
    // ------------------------ +
    // 3 ORE, 4 B => 4 FUEL

    // step = 2, eliminating B
    // 3 ORE, 4 B => 4 FUEL  | x1
    // 1 A        => 1 B     | x4
    // ------------------------ +
    // 3 ORE, 4 A => 4 FUEL

    // step = 3, eliminating A
    // 3 ORE, 4 A => 4 FUEL  | x1
    // 1 ORE      => 4 A     | x1
    // ------------------------ +
    // 4 ORE      => 4 FUEL  ÷ 4
    // 1 ORE      => 1 ORE

    loop {
        let mut current_reaction = reaction.clone();
        let reactants = current_reaction.reactants.clone();
        if reactants.len() == 1 && reactants[0].1 == "ORE" {
            break;
        }
        println!("---------------");
        println!("current_reaction: {:?}", current_reaction);
        let reactant_to_eliminate = reactants.iter().find(|r| r.1 != "ORE").unwrap();
        let other_reaction_index = *reaction_by_product.get(&reactant_to_eliminate.1).unwrap();
        let mut other_reaction = reactions[other_reaction_index].clone();
        // "multiply" phase
        {
            let lcm = get_lcm(other_reaction.product.0, reactant_to_eliminate.0);
            let current_reaction_multiplier = lcm / reactant_to_eliminate.0;
            let other_reaction_multiplier = lcm / other_reaction.product.0;
            current_reaction.product.0 *= current_reaction_multiplier;
            for reactant in current_reaction.reactants.iter_mut() {
                reactant.0 *= current_reaction_multiplier;
            }

            other_reaction.product.0 *= other_reaction_multiplier;
            for reactant in other_reaction.reactants.iter_mut() {
                reactant.0 *= other_reaction_multiplier;
            }
        }
        println!("current_reaction: {:?}", current_reaction);
        println!("other_reaction: {:?}", other_reaction);

        // "add" phase
        let mut new_reaction = Reaction {
            product: current_reaction.product,
            reactants: vec![],
        };
        {
            let mut new_reactants: HashMap<String, usize> = HashMap::new();
            for reactant in current_reaction.reactants.iter() {
                let reactant_name = reactant.1.clone();
                if reactant_name == reactant_to_eliminate.1 {
                    continue;
                }
                let counter = new_reactants.entry(reactant_name).or_insert(0);
                *counter += reactant.0;
            }
            for reactant in other_reaction.reactants.iter() {
                let reactant_name = reactant.1.clone();
                if reactant_name == reactant_to_eliminate.1 {
                    continue;
                }
                let counter = new_reactants.entry(reactant_name).or_insert(0);
                *counter += reactant.0;
            }

            for (key, val) in new_reactants.iter() {
                new_reaction.reactants.push((*val, key.clone()));
            }
        }
        println!("new_reaction: {:?}", new_reaction);

        // "divide" phase
        {
            let mut gcd = new_reaction.product.0;
            for reactant in new_reaction.reactants.clone() {
                gcd = get_gcd(gcd, reactant.0);
            }

            println!("gcd {:?}", gcd);
            new_reaction.product.0 /= gcd;
            for reactant in new_reaction.reactants.iter_mut() {
                reactant.0 /= gcd;
            }
        }
        println!("new_reaction: {:?}", new_reaction);

        reaction = new_reaction.clone();
    }
}
