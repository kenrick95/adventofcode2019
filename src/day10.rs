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
    for (y, line) in inputs.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                locations.push((y, x));
            }
        }
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
        for (j, &other_location) in locations.iter().enumerate().take(locations_length) {
            if i == j {
                continue;
            }
            let gradient = get_gradient(location, other_location);
            set.insert((gradient.0, gradient.1));

            // println!(
            //     "{:?} - {:?}; gradient {:?}",
            //     location, other_location, gradient
            // );
        }
        let set_count = set.len();
        if set_count > max_count {
            max_count = set_count;
            max_count_location = location;
        }

        // println!(">>>>> location {:?} ---> {:?} ", location, set.len());
    }

    println!("Answer pt1: {:?} ", max_count);
    println!("max_count_location: {:?} ", max_count_location);

    // On this max_count_location, destroy all other location from 0deg to top, going clockwise :thinking:
    // i.e. convert it to polar coordinate (origin is 0deg to top), and then sort by angle and then by radius
    // then vaporize one by one; if one have same gradient, kick it to "next rotation"
    {
        // Array of (gradient, point); where gradient (dy, dx) and point = (y, x)
        let mut points: Vec<((isize, isize), (usize, usize))> = vec![];
        for other_location in locations {
            if max_count_location.0 == other_location.0 && max_count_location.1 == other_location.1
            {
                continue;
            }
            let gradient = get_gradient(max_count_location, other_location);
            points.push(((gradient.0, gradient.1), other_location));
        }

        points.sort_by(|a, b| {
            let a_dy = ((*a).0).0 as f64;
            let a_dx = ((*a).0).1 as f64;
            // let a_y = ((*a).1).0;
            // let a_x = ((*a).1).1;
            let dist_a = dist_sq((*a).1, max_count_location);
            let angle_a = -1_f64 * a_dx.atan2(a_dy);
            let b_dy = ((*b).0).0 as f64;
            let b_dx = ((*b).0).1 as f64;
            // let b_y = ((*b).1).0;
            // let b_x = ((*b).1).1;
            let dist_b = dist_sq((*b).1, max_count_location);
            let angle_b = -1_f64 * b_dx.atan2(b_dy);

            // println!(
            //     "a {:?}, {:?}; {:?}, {:?} --> {:?}; {:?}",
            //     a_dy, a_dx, a_y, a_x, angle_a, dist_a
            // );
            // println!(
            //     "b {:?}, {:?}; {:?}, {:?} --> {:?}; {:?}",
            //     b_dy, b_dx, b_y, b_x, angle_b, dist_b
            // );

            let res = angle_a.partial_cmp(&angle_b).unwrap();
            if res != Ordering::Equal {
                res
            } else {
                dist_a.cmp(&dist_b)
            }
        });
        println!("points: {:?}; {:?} ", points.len(), points);

        // Run the "vaporize"

        // (y, x)
        let mut champion = (0, 0);

        let mut iteration = 0;
        let mut mark = vec![false; points.len()];
        let mut global_i = 0;
        while iteration < 10 {
            iteration += 1;
            // println!("Iteration {:?}", iteration);
            let mut i = 0;
            let mut last_gradient = (-1, -1);
            while i < points.len() {
                // println!("Iteration {:?} {:?}", iteration, i);
                let point = points[i];
                // println!("Iteration {:?} {:?}; visiting {:?}", iteration, i, point);

                if mark[i] {
                    // Already blasted
                    i += 1;
                    continue;
                }

                if last_gradient == point.0 {
                    // Same gradient
                    i += 1;
                    continue;
                }

                // Blast
                global_i += 1;
                last_gradient = point.0;
                mark[i] = true;
                i += 1;
                if global_i == 200 {
                    println!(
                        ">>> Iteration {:?} {:?}; blasting {:?}",
                        iteration, i, point
                    );
                    champion = point.1;
                }

                // println!("Iteration {:?} {:?}; blasting {:?}", iteration, i, point);
            }

            if mark.iter().find(|&&m| !m).is_none() {
                // println!("Iteration {:?}: None is false", iteration);
                break;
            }
        }
        println!("champion: {:?}", champion);
        println!("Answer pt2: {:?}", champion.0 + champion.1 * 100);
    }
}

fn get_gradient(location_a: (usize, usize), location_b: (usize, usize)) -> (isize, isize) {
    let dy = location_b.0 as isize - location_a.0 as isize;
    let dx = location_b.1 as isize - location_a.1 as isize;
    let dy_mult = if dy < 0 { -1 } else { 1 };
    let dx_mult = if dx < 0 { -1 } else { 1 };

    let gcd: isize = get_gcd((dx * dx_mult) as usize, (dy * dy_mult) as usize) as isize;

    // println!("gcd {:?}, {:?}; {:?}, {:?}; {:?}", location_a, location_b, dy, dx, gcd);

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

fn dist_sq(a: (usize, usize), b: (usize, usize)) -> usize {
    (a.0 as isize - b.0 as isize).pow(2) as usize + (a.1 as isize - b.1 as isize).pow(2) as usize
}
