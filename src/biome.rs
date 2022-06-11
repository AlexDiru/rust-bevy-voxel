use crate::biome::BiomeType::{Flat, PerlinMountains, Quarry};

#[derive(Copy, Clone)]
pub enum BiomeType {
    PerlinMountains,
    Flat,
    Quarry,
}

struct BiomeParam {
    pub biome_type: BiomeType,
    pub biome_chance: f64,
}

pub struct BiomeStrength {
    pub biome_type: BiomeType,
    pub biome_strength: f64,
}

const BIOME_PARAMS_ARR: [BiomeParam; 3] = [
    BiomeParam {
        biome_type: PerlinMountains,
        biome_chance: 0.5,
    },
    BiomeParam {
        biome_type: Flat,
        biome_chance: 0.5,
    },
    BiomeParam {
        biome_type: Quarry,
        biome_chance: 0.5,
    }
];

pub fn get_random_biome(rand_val: f64) -> [BiomeStrength; 3] {
    let mut max_biome_chance = 0.0;

    for i in 0..BIOME_PARAMS_ARR.len() {
        max_biome_chance += BIOME_PARAMS_ARR[i].biome_chance;
    }

    let rand_val = rand_val * max_biome_chance;

    let mut prev_sum = 0.0;
    let mut sum = 0.0;

    for current_biome in 0..BIOME_PARAMS_ARR.len() as i32 {
        sum += BIOME_PARAMS_ARR[current_biome as usize].biome_chance;
        if rand_val <= sum {
            let mut prev_biome = current_biome - 1;
            if prev_biome < 0 {
                prev_biome = BIOME_PARAMS_ARR.len() as i32 - 1;
            }
            let mut next_biome = current_biome + 1;
            if next_biome >= BIOME_PARAMS_ARR.len() as i32 {
                next_biome = 0;
            }

            // prev_sum = 0.5
            // sum = 1.0
            // rand_val = 0.75 (max strength because centre of biome)
            // desired: strength_val = 1
            // rand_val - prev_sum = 0.25
            // sum - prev_sum = 0.5
            // bs = (sum - prev_sum)/2 = 0.25
            // rand_val = 0.75: bs - |(rand_val - prev_sum) - bs| = 0.25 - 0.25 = 0 = 0.25/bs = 1
            // rand_val = 0.9:  bs - |(rand_val - prev_sum) - bs| = 0.4 - 0.25 = 0.15 = 0.1
            // rand_val = 0.6:  bs - |(rand_val - prev_sum) - bs| = 0.1 - 0.25 = 0.15 = 0.1
            let bs = (sum - prev_sum) / 2.0;
            let current_biome_strength = (bs - ((rand_val - prev_sum) - bs).abs())/bs;
            let mut next_biome_strength = 0.0;
            let mut prev_biome_strength = 0.0;

            if current_biome_strength < 0.7 {
                // Factor in other biome strength
                // TODO
                next_biome_strength = (1.0 - current_biome_strength)/2.0;
                prev_biome_strength = (1.0 - current_biome_strength)/2.0;
            }

            return [
                BiomeStrength {
                    biome_type: BIOME_PARAMS_ARR[current_biome as usize].biome_type,
                    biome_strength: current_biome_strength
                },
                BiomeStrength {
                    biome_type: BIOME_PARAMS_ARR[next_biome as usize].biome_type,
                    biome_strength: next_biome_strength
                },
                BiomeStrength {
                    biome_type: BIOME_PARAMS_ARR[prev_biome as usize].biome_type,
                    biome_strength: prev_biome_strength
                }
            ]
        }
        prev_sum = sum;
    }

    panic!("AHHHHHH")
}