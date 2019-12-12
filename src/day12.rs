use super::utils::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
struct Particle {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
}
#[derive(Debug, Clone, PartialEq)]
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
    run_simulation(&particles);
}

fn run_simulation(particles: &Vec<Particle>) {
    let mut t = 0;
    let mut state = State {
        particles: particles.clone(),
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

fn apply_gravity(state: &State) -> State {
    // (particle_index, (dx, dy, dz))
    let mut delta_v: HashMap<usize, (isize, isize, isize)> = HashMap::new();
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

fn get_gravity(
    particle_a: &Particle,
    particle_b: &Particle,
) -> ((isize, isize, isize), (isize, isize, isize)) {
    let delta_i = (
        cmp_value(particle_a.position.0, particle_b.position.0),
        cmp_value(particle_a.position.1, particle_b.position.1),
        cmp_value(particle_a.position.2, particle_b.position.2),
    );
    (delta_i, (-delta_i.0, -delta_i.1, -delta_i.2))
}
fn cmp_value(value_a: isize, value_b: isize) -> isize {
    if value_a > value_b {
        -1
    } else if value_a < value_b {
        1
    } else {
        0
    }
}

fn get_total_energy(state: &State) -> isize {
    let mut answer = 0;
    for particle in state.particles.iter() {
        answer += get_potential_energy(particle) * get_kinetic_energy(particle);
    }
    answer
}
fn get_potential_energy(particle: &Particle) -> isize {
    particle.position.0.abs() + particle.position.1.abs() + particle.position.2.abs()
}
fn get_kinetic_energy(particle: &Particle) -> isize {
    particle.velocity.0.abs() + particle.velocity.1.abs() + particle.velocity.2.abs()
}
