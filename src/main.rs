use rand::{thread_rng, Rng, rngs::ThreadRng};
use std::fs::File;
use std::io::prelude::*;

const PICKS: usize = 5; // Numbers in one draw. Montana Cash = 5
const DRAWS: usize = 100; // Number of times you're playing

fn main() {
    println!("Dockerized Lotto, version 2.0");
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
    let mut matches: [u32; PICKS+1] = [0; PICKS+1];
    
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
    for i in 0..PICKS+1 {
        println!("Matched {}: {}", i, matches[i]);
        match_sum += matches[i];
    }

    assert_eq!(DRAWS as u32, match_sum);

    match file_output(winners, draws) {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
}

fn file_output(winners: [u8; PICKS], draws: Vec<Vec<u8>>) -> std::io::Result<()> {
    let mut file = File::create("audit_trail.txt")?;
    let mut output = String::new();

    output += "Winners: ";
    for winner in winners {
        let mut winner_string = winner.to_string();
        winner_string += " ";
        output += &winner_string;
    }
    output += "\n\nDraws:\n";
    for draw in draws {
        for dr in draw {
            let mut draw_string = dr.to_string();
            draw_string += " ";
            output += &draw_string
        }
        output += "\n";
    }
    output += "\n";

    file.write_all(output.as_bytes())?;

    Ok(())
}