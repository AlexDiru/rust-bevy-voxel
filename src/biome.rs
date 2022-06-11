use crate::biome::BiomeType::{FLAT, PERLIN_MOUNTAINS, QUARRY};

#[derive(Copy, Clone)]
pub enum BiomeType {
    PERLIN_MOUNTAINS,
    FLAT,
    QUARRY,
}

struct BiomeParam {
    pub biomeType: BiomeType,
    pub biomeChance: f64,
}

pub struct BiomeStrength {
    pub biomeType: BiomeType,
    pub biomeStrength: f64,
}

const biome_params_arr: [BiomeParam; 3] = [
    BiomeParam {
        biomeType: PERLIN_MOUNTAINS,
        biomeChance: 0.5,
    },
    BiomeParam {
        biomeType: FLAT,
        biomeChance: 0.5,
    },
    BiomeParam {
        biomeType: QUARRY,
        biomeChance: 0.5,
    }
];

pub fn get_random_biome(rand_val: f64) -> [BiomeStrength; 3] {
    let mut max_biome_chance = 0.0;

    for i in 0..biome_params_arr.len() {
        max_biome_chance += biome_params_arr[i].biomeChance;
    }

    let rand_val = rand_val * max_biome_chance;

    let mut prev_sum = 0.0;
    let mut sum = 0.0;

    for current_biome in 0..biome_params_arr.len() as i32 {
        sum += biome_params_arr[current_biome as usize].biomeChance;
        if rand_val <= sum {
            let mut prev_biome = current_biome - 1;
            if prev_biome < 0 {
                prev_biome = biome_params_arr.len() as i32 - 1;
            }
            let mut nextBiome = current_biome + 1;
            if nextBiome >= biome_params_arr.len() as i32 {
                nextBiome = 0;
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
            let currentBiomeStrength = (bs - ((rand_val - prev_sum) - bs).abs())/bs;
            let mut nextBiomeStrength = 0.0;
            let mut prevBiomeStrength = 0.0;

            if currentBiomeStrength < 0.7 {
                // Factor in other biome strength
                // TODO
                nextBiomeStrength = (1.0 - currentBiomeStrength)/2.0;
                prevBiomeStrength = (1.0 - currentBiomeStrength)/2.0;
            }

            return [
                BiomeStrength {
                    biomeType: biome_params_arr[current_biome as usize].biomeType,
                    biomeStrength: currentBiomeStrength
                },
                BiomeStrength {
                    biomeType: biome_params_arr[nextBiome as usize].biomeType,
                    biomeStrength: nextBiomeStrength
                },
                BiomeStrength {
                    biomeType: biome_params_arr[prev_biome as usize].biomeType,
                    biomeStrength: prevBiomeStrength
                }
            ]
        }
        prev_sum = sum;
    }

    panic!("AHHHHHH")
}