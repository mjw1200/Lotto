use rand::{thread_rng, Rng, rngs::ThreadRng};

const PICKS: usize = 5; // Numbers in one draw. MT Cash = 5
const DRAWS: usize = 3; // Number of draws

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
    println!("Draws: {:?}", draws);

    let mut winners: [u8; PICKS] = [0; PICKS];

    for i in 0..PICKS {
        winners[i] = rng.gen_range(1..45);
    }
    winners.sort();
    println!("Winners: {:?}", winners);

    for i in 0..DRAWS {
        let mut matches: u8 = 0;

        for 
        j in 0..PICKS {
            if winners[j] == draws[i][j] {
                matches += 1;
            }
        }

        println!("{} matches", matches);
    } 
}
