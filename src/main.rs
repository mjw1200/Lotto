use rand::{thread_rng, Rng, rngs::ThreadRng};

const PICKS: usize = 5; // Numbers in one draw. MT Cash = 5
const DRAWS: usize = 1000000; // Number of draws

fn main() {
    let mut rng = thread_rng();
    let draws = generate(&mut rng);
    check(&mut rng, draws);
}

fn generate(rng: &mut ThreadRng) -> Vec<Vec<u8>> {
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

fn check(rng: &mut ThreadRng, draws: Vec<Vec<u8>>) {
    let mut match_two: u32 = 0;
    let mut match_three: u32 = 0;
    let mut match_four: u32 = 0;
    let mut match_five: u32 = 0;
    
    // println!("Draws: {:?}", draws);

    let mut winners: [u8; PICKS] = [0; PICKS];

    for i in 0..PICKS {
        winners[i] = rng.gen_range(1..45);
    }
    winners.sort();
    // println!("Winners: {:?}", winners);

    for i in 0..DRAWS {
        let mut matches: u8 = 0;

        for j in 0..PICKS {
            if winners[j] == draws[i][j] {
                matches += 1;
            }
        }

        if matches == 2 {
            match_two += 1;
        }
        else if matches == 3 {
            match_three += 1;
        }
        else if matches == 4 {
            match_four += 1;
        }
        else if matches == 5 {
            match_five += 1;
        }        
    }

    println!("Matched two: {}", match_two);
    println!("Matched three: {}", match_three);
    println!("Matched four: {}", match_four);
    println!("Matched five: {}", match_five);
}
