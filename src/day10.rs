use super::utils::get_string_from_stdio;
use std::cmp::Ordering;
use std::collections::HashSet;

pub fn main() {
    // get input
    let mut inputs: Vec<String> = vec![];
    loop {
        let input = get_string_from_stdio();
        if input.trim() == "" {
            break;
        }
        inputs.push(input.trim().to_string());
    }
    // (y,x)
    let mut locations: Vec<(usize, usize)> = vec![];
    let mut y = 0;
    for line in inputs {
        let mut x = 0;
        for ch in line.chars() {
            if ch == '#' {
                locations.push((y, x));
            }
            x += 1;
        }
        y += 1;
    }

    // for each '#', check gradient of all other '#', count different number of gradients
    // Tricky: floating point number --> save as fraction? Need to GCD
    let mut max_count = 0;
    let mut max_count_location = (0, 0);
    println!("locations {:?}", locations);
    let locations_length = locations.len();
    for i in 0..locations_length {
        let location = locations[i];
        let mut set: HashSet<(isize, isize)> = HashSet::new();
        for j in 0..locations_length {
            if i == j {
                continue;
            }
            let other_location = locations[j];
            let gradient = get_gradient(location, other_location);
            set.insert(gradient);

            // println!(
            //     "{:?} - {:?}; gradient {:?}",
            //     location, other_location, gradient
            // );
        }
        let set_count = set.len();
        if set_count > max_count {
            max_count = set_count;
            max_count_location = location.clone();
        }

        // println!(">>>>> location {:?} ---> {:?} ", location, set.len());
    }

    println!("Answer pt1: {:?} ", max_count);
    println!("max_count_location: {:?} ", max_count_location);

    // On this max_count_location, destroy all other location from 0deg to top, going clockwise :thinking:
    // i.e. sort by gradient (y/x)
    // then vaporize one by one; if one have same gradient, kick it to "next rotation"
    {
        // Array of (gradient, point); where gradient (dy, dx) and point = (y, x)
        let mut points: Vec<((isize, isize), (usize, usize))> = vec![];
        for j in 0..locations_length {
            let other_location = locations[j];
            if max_count_location.0 == other_location.0 && max_count_location.1 == other_location.1
            {
                continue;
            }
            let gradient = get_gradient(max_count_location, other_location);
            points.push((gradient, other_location));
        }

        points.sort_by(|a, b| {
            let a_dy = ((*a).0).0;
            let a_dx = ((*a).0).1;

            let b_dy = ((*b).0).0;
            let b_dx = ((*b).0).1;

            if a_dx == 0 && b_dx == 0 {
                // Infinite gradient; by right their dy will be 1 // TODO: Verify this theory
                // TODO: priorize by length of point to  `max_count_location`
                
                return Ordering::Equal;
            } else if a_dx == 0 {
                // Infinite gradient
                return Ordering::Less;
            } else if b_dx == 0 {
                // Infinite gradient
                return Ordering::Greater;
            } else if a_dy == b_dy && a_dx == b_dx {
                // TODO: priorize by length of point to  `max_count_location`
                return Ordering::Equal;
            } else {
                let a_grad = a_dy as f64 / a_dx as f64;
                let b_grad = b_dy as f64 / b_dx as f64;
                if a_grad < b_grad {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
        });
        // TODO: Run the "vaporize"

        println!("points: {:?} ", points);
    }
}

fn get_gradient(location_a: (usize, usize), location_b: (usize, usize)) -> (isize, isize) {
    let dy = location_a.0 as isize - location_b.0 as isize;
    let dx = location_a.1 as isize - location_b.1 as isize;
    let dy_mult = if dy < 0 { -1 } else { 1 };
    let dx_mult = if dx < 0 { -1 } else { 1 };

    let gcd: isize = get_gcd((dx * dx_mult) as usize, (dy * dy_mult) as usize) as isize;

    (dy / gcd, dx / gcd)
}

// https://en.wikipedia.org/wiki/Euclidean_algorithm
fn get_gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        get_gcd(b, a % b)
    }
}
