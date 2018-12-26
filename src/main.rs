extern crate asteroidroulette_lib;
extern crate rand;

use asteroidroulette_lib::*;

use rand::distributions::{Distribution, Uniform};
use std::io::{stdout, Write};
use std::{thread, time};

fn main() {
    let dice = Uniform::from(1..7);
    let mut rng = rand::thread_rng();

    let mut state = State::new();
    print!("You rolled ");
    while state.victory().is_none() {
        let roll = dice.sample(&mut rng);
        print!("{}", roll);
        let _ = stdout().flush();
        let change = state.update(roll);
        if let (PositionDelta::Nothing, StateDelta::Nothing) = change {
        } else {
            println!("...!");
        }
        thread::sleep(time::Duration::from_millis(800));
        match change {
            (PositionDelta::Forward(x), _) => println!("You moved forward to {}.", x),
            (PositionDelta::Jump(x), _) => println!("You jumped to {}!", x),
            (_, StateDelta::Shield) => println!("You lost your shields!"),
            (PositionDelta::Stuck(x), _) => {
                println!("You would have moved to {}, but you are stuck.", x)
            }
            (PositionDelta::ToZero, _) => println!("An asteroid moved you to position 0!"),
            (_, StateDelta::Jumping) => println!("You will jump on the next turn!"),
            (_, StateDelta::Stuck) => println!("You are stuck in space!"),
            (_, StateDelta::Unstuck) => println!("You are no longer stuck in space!"),
            (_, StateDelta::Death) => println!("You died."),
            (PositionDelta::Nothing, StateDelta::Nothing) => print!(", "),
        }
        if let (PositionDelta::Nothing, StateDelta::Nothing) = change {
        } else if state.victory().is_none() {
            thread::sleep(time::Duration::from_millis(1000));
            print!("You rolled ");
        }
    }

    let result = match state.cause_of_death {
        None => "You won!",
        Some(CauseOfDeath::Shields) => "You got hit by an asteroid without having shields.",
        Some(CauseOfDeath::Ceres) => "You got hit by Ceres.",
        Some(CauseOfDeath::Both) => "You got hit by Ceres... and not having shields didn't help.",
    };

    println!("{}", result);
}
