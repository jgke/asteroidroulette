extern crate asteroidroulette_lib;
extern crate rand;

use asteroidroulette_lib::*;

use rand::distributions::{Distribution, Uniform};

#[test]
fn base_game() {
    let mut state = State::new();
    (1..7).for_each(|x| { state.update(x); });
    assert_eq!(state.position, 6);
}
#[test]
fn jump_to_6() {
    let mut state = State::new();
    [4, 4, 6].iter().for_each(|x| { state.update(*x); });
    assert_eq!(state.position, 6);
}
#[test]
fn death_by_ceres() {
    let mut state = State::new();
    [6, 6].iter().for_each(|x| { state.update(*x); });
    assert_eq!(state.dead, true);
    assert_eq!(state.cause_of_death, Some(CauseOfDeath::Ceres));
}
#[test]
fn death_by_no_shields() {
    let mut state = State::new();
    [1, 1, 2, 2].iter().for_each(|x| { state.update(*x); });
    assert_eq!(state.dead, true);
    assert_eq!(state.cause_of_death, Some(CauseOfDeath::Shields));
}
#[test]
fn no_shields_but_alive() {
    let mut state = State::new();
    [2, 2, 1, 1].iter().for_each(|x| { state.update(*x); });
    assert_eq!(state.dead, false);
}
#[test]
fn stuck_at_5() {
    let mut state = State::new();
    [1, 2, 3, 4, 5, 5, 6].iter().for_each(|x| { state.update(*x); });
    assert_eq!(state.position, 5);
}
#[test]
fn stuck_and_unstuck() {
    let mut state = State::new();
    [1, 2, 3, 4, 5, 5, 6, 5, 6]
        .iter()
        .for_each(|x| { state.update(*x); });
    assert_eq!(state.position, 6);
}
#[test]
fn history_is_cleared_after_hitting_asteroid() {
    let mut state = State::new();
    [1, 1, 1].iter().for_each(|x| { state.update(*x); });
    assert_eq!(state.dead, false);
}
#[test]
fn jumping_works_when_stuck() {
    let mut state = State::new();
    [5, 5, 4, 4, 6].iter().for_each(|x| { state.update(*x); });
    assert_eq!(state.position, 6);
}
#[test]
fn death_by_both_ceres_and_no_shields_at_once() {
    let mut state = State::new();
    [1, 1, 6, 6].iter().for_each(|x| { state.update(*x); });
    assert_eq!(state.dead, true);
    assert_eq!(state.cause_of_death, Some(CauseOfDeath::Both));
}
#[test]
fn no_double_jumping() {
    let mut state = State::new();
    [4, 4, 3, 2].iter().for_each(|x| { state.update(*x); });
    assert_eq!(state.position, 3);
}

#[test]
fn victory_probabilities() {
    let dice = Uniform::from(1..7);
    let mut rng = rand::thread_rng();
    let mut ends = (0, 0, 0, 0);

    for _ in 0..10000 {
        let mut state = State::new();
        while state.victory().is_none() {
            state.update(dice.sample(&mut rng));
        }
        match (state.victory().unwrap(), state.cause_of_death) {
            (true, _) => ends.0 += 1,
            (false, Some(CauseOfDeath::Shields)) => ends.1 += 1,
            (false, Some(CauseOfDeath::Ceres)) => ends.2 += 1,
            (false, Some(CauseOfDeath::Both)) => ends.3 += 1,
            (false, _) => panic!("Unreachable"),
        }
    }

    let sum = (ends.0 as f32) + (ends.1 as f32) + (ends.2 as f32) + (ends.3 as f32);
    println!("Victories: {}", (ends.0 as f32) / sum);
    println!("Death by shield: {}", (ends.1 as f32) / sum);
    println!("Death by Ceres: {}", (ends.2 as f32) / sum);
    println!("Death by both at once: {}", (ends.3 as f32) / sum);
}
