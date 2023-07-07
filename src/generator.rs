use crate::DRAWS;
use crate::PICKS;
use rand::{Rng, rngs::ThreadRng};

pub struct Generator {
}

impl Generator {
    pub fn generate(rng: &mut ThreadRng) -> Vec<Vec<u8>> {
        let mut draws: Vec<Vec<u8>> = Vec::new();
        
        for _i in 0..DRAWS {
            let mut draw = Vec::new();
    
            for _j in 0..PICKS {
                draw.push(rng.gen_range(1..45));
            }
    
            draw.sort();
            draws.push(draw);
        } 
    
        draws
    }
}