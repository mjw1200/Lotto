use rand::{thread_rng, Rng, rngs::ThreadRng};

const PICKS: usize = 5; // Numbers in one draw. Montana Cash = 5
const DRAWS: usize = 100; // Number of times you're playing

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
    let mut matches: [u32; PICKS] = [0; PICKS];
    
    // println!("Draws: {:?}", draws);

    let mut winners: [u8; PICKS] = [0; PICKS];

    for i in 0..PICKS {
        winners[i] = rng.gen_range(1..45);
    }
    winners.sort();
    // println!("Winners: {:?}", winners);

    for i in 0..DRAWS {
        let mut match_count: usize = 0;

        for j in 0..PICKS {
            if winners[j] == draws[i][j] {
                match_count += 1;
            }
        }

        matches[match_count] += 1;
    }

    println!("Out of {} draws:\n", DRAWS);
    let mut match_sum: u32 = 0;
    for i in 0..PICKS {
        println!("Matched {}: {}", i, matches[i]);
        match_sum += matches[i];
    }

    assert_eq!(DRAWS as u32, match_sum);
}
