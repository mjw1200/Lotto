use rand::{thread_rng};

const PICKS: usize = 5; // Numbers in one draw. Montana Cash = 5
const DRAWS: usize = 100; // Number of times you're playing 

mod generator;
mod checker;

fn main() {
    println!("Dockerized Lotto, version 3.3");
    let mut rng = thread_rng();
    
    let draws = generator::Generator::generate(&mut rng);

    checker::Checker::check(&mut rng, draws);
}
