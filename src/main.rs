use rand::{thread_rng, Rng};

const PICKS: usize = 5; // Numbers in one draw. MT Cash = 5
const DRAWS: usize = 5; // Number of draws

fn main() {
    let vec = generate();
    println!("{:?}", vec);
}

fn generate() -> Vec<Vec<u8>> {
    let mut rng = thread_rng();

    let mut vec: Vec<Vec<u8>> = Vec::new();
    
    for _i in 0..DRAWS {
        let mut draw = Vec::new();

        for _j in 0..PICKS {
            draw.push(rng.gen_range(1..45));
        }

        draw.sort();
        vec.push(draw);
    } 

    return vec;
}
