use super::utils::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Particle {
    position: (i16, i16, i16),
    velocity: (i16, i16, i16),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    particles: Vec<Particle>,
}

pub fn main() {
    let mut particles: Vec<Particle> = vec![];
    let re = Regex::new(r"^<x=([-0-9]+), y=([-0-9]+), z=([-0-9]+)>$").unwrap();
    loop {
        let input = get_string_from_stdio().trim().to_string();
        if input == "" {
            break;
        }
        let captures = re.captures(&input).unwrap();
        particles.push(Particle {
            position: (
                captures.get(1).unwrap().as_str().parse().unwrap(),
                captures.get(2).unwrap().as_str().parse().unwrap(),
                captures.get(3).unwrap().as_str().parse().unwrap(),
            ),
            velocity: (0, 0, 0),
        });
    }
    run_simulation_pt1(&particles);
    run_simulation_pt2(&particles);
}

fn run_simulation_pt1(particles: &[Particle]) {
    let mut t = 0;
    let mut state = State {
        particles: particles.to_owned(),
    };
    while t < 1000 {
        // println!("[it={:?}]: {:?}", t, state);
        let mut new_state = apply_gravity(&state);
        // println!("[it={:?}]: {:?}", t, new_state);
        new_state = apply_velocity(&new_state);
        // println!("[it={:?}]: {:?}", t, new_state);
        state = new_state;
        t += 1;
        // println!();
    }

    println!("Answer part 1: {:?}", get_total_energy(&state));
}

/**
 * Hint: Each axis are independent of each other, so each axis can have their own cycle. Once found, just LCM it.
 * https://www.reddit.com/r/adventofcode/comments/e9jxh2/help_2019_day_12_part_2_what_am_i_not_seeing/
 * */
fn run_simulation_pt2(particles: &[Particle]) {
    let mut t = 1;
    let mut state = State {
        particles: particles.to_owned(),
    };
    let mut cycle_found = vec![false, false, false];
    // axis --> Array<(position, velocity)>
    let mut seen_by_axis: Vec<HashSet<Vec<(i16, i16)>>> = vec![HashSet::new(); 3];
    for (i, seen_axis) in seen_by_axis.iter_mut().enumerate() {
        seen_axis.insert(
            state
                .particles
                .iter()
                .map(|p| get_axis_values(p, i))
                .collect(),
        );
    }
    loop {
        // println!("[it={:?}]: {:?}", t, state);
        let mut new_state = apply_gravity(&state);
        // println!("[it={:?}]: {:?}", t, new_state);
        new_state = apply_velocity(&new_state);
        // println!("[it={:?}]: {:?}", t, new_state);
        state = new_state;

        for i in 0..3 {
            if cycle_found[i] {
                continue;
            }
            let value = state
                .particles
                .iter()
                .map(|p| get_axis_values(p, i))
                .collect();
            if seen_by_axis[i].contains(&value) {
                cycle_found[i] = true;
                continue;
            }
            seen_by_axis[i].insert(value);
        }
        if !cycle_found.contains(&false) {
            break;
        }
        t += 1;
    }
    println!("t={:?}", t);
    let mut answer = 1;
    for seen in seen_by_axis {
        answer = get_lcm(answer, seen.len());
        println!("seen_by_axis: {:?} --> {}", seen.len(), answer);
    }

    println!("Answer part 2: {:?}", answer);
}

fn apply_gravity(state: &State) -> State {
    // (particle_index, (dx, dy, dz))
    let mut delta_v: HashMap<usize, (i16, i16, i16)> = HashMap::new();
    let particles = state.particles.clone();

    for i in 0..particles.len() {
        delta_v.insert(i, (0, 0, 0));
    }
    for i in 0..particles.len() {
        for j in 0..particles.len() {
            if i >= j {
                continue;
            }
            // Calculate "gravity"
            let (delta_i, delta_j) = get_gravity(&particles[i], &particles[j]);

            let v_i = *delta_v.get(&i).unwrap();
            delta_v.insert(i, (v_i.0 + delta_i.0, v_i.1 + delta_i.1, v_i.2 + delta_i.2));

            let v_j = *delta_v.get(&j).unwrap();
            delta_v.insert(j, (v_j.0 + delta_j.0, v_j.1 + delta_j.1, v_j.2 + delta_j.2));
        }
    }
    let mut new_state = state.clone();
    for i in 0..particles.len() {
        let d_v = *delta_v.get(&i).unwrap();
        new_state.particles[i].velocity = (
            new_state.particles[i].velocity.0 + d_v.0,
            new_state.particles[i].velocity.1 + d_v.1,
            new_state.particles[i].velocity.2 + d_v.2,
        );
    }
    new_state
}

fn apply_velocity(state: &State) -> State {
    let mut new_state = state.clone();
    for particle in new_state.particles.iter_mut() {
        particle.position = (
            particle.position.0 + particle.velocity.0,
            particle.position.1 + particle.velocity.1,
            particle.position.2 + particle.velocity.2,
        );
    }
    new_state
}

fn get_gravity(particle_a: &Particle, particle_b: &Particle) -> ((i16, i16, i16), (i16, i16, i16)) {
    let delta_i = (
        cmp_value(particle_a.position.0, particle_b.position.0),
        cmp_value(particle_a.position.1, particle_b.position.1),
        cmp_value(particle_a.position.2, particle_b.position.2),
    );
    (delta_i, (-delta_i.0, -delta_i.1, -delta_i.2))
}
fn cmp_value(value_a: i16, value_b: i16) -> i16 {
    if value_a > value_b {
        -1
    } else if value_a < value_b {
        1
    } else {
        0
    }
}

fn get_total_energy(state: &State) -> i16 {
    let mut answer = 0;
    for particle in state.particles.iter() {
        answer += get_potential_energy(particle) * get_kinetic_energy(particle);
    }
    answer
}
fn get_potential_energy(particle: &Particle) -> i16 {
    particle.position.0.abs() + particle.position.1.abs() + particle.position.2.abs()
}
fn get_kinetic_energy(particle: &Particle) -> i16 {
    particle.velocity.0.abs() + particle.velocity.1.abs() + particle.velocity.2.abs()
}
fn get_axis_values(particle: &Particle, index: usize) -> (i16, i16) {
    match index {
        0 => (particle.position.0, particle.velocity.0),
        1 => (particle.position.1, particle.velocity.1),
        2 => (particle.position.2, particle.velocity.2),
        _ => (0, 0),
    }
}
