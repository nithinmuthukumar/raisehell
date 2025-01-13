use std::thread::current;

use num::integer::binomial;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn how_many_hellraisers(
    triggers: u32,
    gy_size: u32,
    seasons: u32,
    beacons: u32,
    flameshapers: u32,
) -> Vec<f64> {
    let max_hellraisers = 2 * seasons + 2 * beacons + flameshapers;
    let mut probabilities = vec![0.0; (max_hellraisers + 1) as usize];

    // Recursive function to calculate probabilities
    fn calculate(
        triggers: u32,
        gy_size: u32,
        seasons: u32,
        beacons: u32,
        flameshapers: u32,
        current_hellraisers: u32,
        beacon_active: bool,
        probability: f64,
        probabilities: &mut Vec<f64>,
    ) {
        // Base case: If no more triggers or all cards processed, store the result
        if triggers == 0 || (seasons == 0 && flameshapers == 0) {
            probabilities[current_hellraisers as usize] += probability;
            return;
        }
        if gy_size < 3 {
            probabilities[current_hellraisers as usize] += probability;
            return;
        }

        // Total combinations of choosing 3 cards from the graveyard
        let total_combinations = binomial(gy_size, 3) as f64;

        // Process combinations of Seasons, Beacons, and Flameshapers here
        for i in 0..=seasons.min(3) {
            for j in 0..=(3 - i).min(beacons) {
                for k in 0..=(3 - i - j).min(flameshapers) {
                    let m = 3 - i - j - k;

                    if m > gy_size - (seasons + beacons + flameshapers) {
                        continue;
                    }

                    let combination_count = binomial(seasons, i)
                        * binomial(beacons, j)
                        * binomial(flameshapers, k)
                        * binomial(gy_size - (seasons + beacons + flameshapers), m);

                    let trigger_probability = combination_count as f64 / total_combinations;

                    let mut new_hellraisers = 0;
                    let mut new_beacon_active = beacon_active;

                    // Process Seasons first
                    if i > 0 {
                        new_hellraisers += 2;
                        if beacon_active {
                            new_hellraisers += 2; // Double the value if Beacon is active
                            new_beacon_active = false;
                        }
                    }

                    // Process Flameshapers, but only if no Seasons are selected
                    if k > 0 && i == 0 {
                        new_hellraisers += 1;
                    }

                    // Process Beacons
                    if j > 0 && i == 0 {
                        new_beacon_active = true;
                    }

                    calculate(
                        triggers + new_hellraisers - 1,
                        gy_size - 3,
                        seasons - i,
                        beacons - j,
                        flameshapers - k,
                        current_hellraisers + new_hellraisers,
                        new_beacon_active,
                        probability * trigger_probability,
                        probabilities,
                    );
                }
            }
        }
    }

    calculate(
        triggers,
        gy_size,
        seasons,
        beacons,
        flameshapers,
        0,
        false,
        1.0,
        &mut probabilities,
    );

    probabilities
}

pub fn simulate_hellraiser_trigger(hits: u32, gy_size: u32) -> bool {
    let mut rng = thread_rng();

    let mut graveyard: Vec<bool> = vec![true; hits as usize];
    graveyard.extend(vec![false; (gy_size - hits) as usize]);

    graveyard.shuffle(&mut rng);

    // Simulate drawing 3 cards
    let drawn = &graveyard[0..3];
    return drawn.iter().any(|&card| card);
}

pub fn chances_of_hit(hits: u32, mut gy_size: u32, triggers: u32) -> f64 {
    let mut miss_prob = 1.0;

    for _ in 0..triggers {
        if gy_size < 3 {
            if hits != 0 {
                miss_prob = 0.;
            } else {
                miss_prob = 1.;
            }
            break;
        }

        let total_combinations = binomial(gy_size, 3) as f64;
        let non_hit_combinations = binomial(gy_size - hits, 3) as f64;
        let non_hit_prob = non_hit_combinations / total_combinations;

        miss_prob *= non_hit_prob;
        gy_size -= 3;
    }

    1.0 - miss_prob
}
#[cfg(test)]
mod tests {
    use crate::{chances_of_hit, how_many_hellraisers, how_many_hellraisers_advanced};

    #[test]
    fn test_hellraiser_count() {
        let triggers = 1;
        let gy_size = 15;
        let seasons = 3;
        let beacons = 1;
        let flameshapers = 1;

        let p1 = how_many_hellraisers(triggers, gy_size, seasons, beacons, flameshapers);
        dbg!(p1.iter().sum::<f64>());
        let p2 = how_many_hellraisers_advanced(triggers, gy_size, seasons, beacons, flameshapers);
        dbg!(p2.iter().sum::<f64>());
        assert_eq!(p1, p2)
    }
    #[test]
    fn test_hit_chances() {
        let hits = 2;
        let gy_size = 14;
        let triggers = 3;
        let chance = chances_of_hit(hits, gy_size, triggers);
    }
}
