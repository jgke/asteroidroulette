extern crate asteroidroulette_lib;
extern crate rand;

use asteroidroulette_lib::*;

use rand::distributions::{Distribution, Uniform};

fn main() {
    let dice = Uniform::from(1..7);
    let mut rng = rand::thread_rng();

    let mut state = State::new();
    while state.victory().is_none() {
        state.update(dice.sample(&mut rng));
    }

    let result = match state.cause_of_death {
        None => "You won!",
        Some(CauseOfDeath::Shields) => "You got hit by an asteroid without having shields.",
        Some(CauseOfDeath::Ceres) => "You got hit by Ceres.",
        Some(CauseOfDeath::Both) => "You got hit by Ceres... and not having shields didn't help.",
    };

    println!("{}", result);
}
