pub fn how_many_hellraisers(triggers: u32, gy_size: u32, seasons: u32) -> Vec<f64> {
    let max_hellraisers = 2 * seasons; // Maximum Hellraisers generated by Seasons
    let mut probabilities = vec![0.0; (max_hellraisers + 1) as usize];

    // Recursive function to calculate probabilities
    fn calculate(
        triggers: u32,
        gy_size: u32,
        seasons: u32,
        current_hellraisers: u32,
        probability: f64,
        probabilities: &mut Vec<f64>,
    ) {
        // Base case: If there are no more triggers, store the result
        if triggers == 0 || seasons == 0 {
            probabilities[current_hellraisers as usize] += probability;
            return;
        }
        if gy_size < 3 {
            //already made sure there is atleast one season in the previous if statement
            probabilities[(current_hellraisers + 2) as usize] += probability;
            return;
        }

        // Total combinations of choosing 3 cards from the graveyard
        let total_combinations = binomial(gy_size, 3) as f64;

        //All cases of exiling i seasons from the graveyard
        for i in 1..=seasons.min(3) {
            let season_combinations = binomial(seasons, i) * binomial(gy_size - seasons, 3 - i);
            let season_prob = season_combinations as f64 / total_combinations;
            calculate(
                triggers + 1,
                gy_size - 3,
                seasons - i,
                current_hellraisers + 2,
                probability * season_prob,
                probabilities,
            )
        }

        // Case 4: Missing a hit (no Seasons)
        let non_hit_combinations = binomial(gy_size - seasons, 3) as f64;

        let non_hit_prob = non_hit_combinations / total_combinations;

        calculate(
            triggers - 1,
            gy_size - 3,
            seasons,
            current_hellraisers,
            probability * non_hit_prob,
            probabilities,
        );
    }

    calculate(triggers, gy_size, seasons, 0, 1.0, &mut probabilities);

    probabilities
}
