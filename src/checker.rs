use crate::DRAWS;
use crate::PICKS;
use rand::{Rng, rngs::ThreadRng};
use std::fs::File;
use std::io::Write;

pub struct Checker {
}

impl Checker {
    pub fn check(rng: &mut ThreadRng, draws: Vec<Vec<u8>>) {
        let mut matches: [u32; PICKS+1] = [0; PICKS+1];
        let mut win: u32 = 0;
        
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
        let mut value: u32;
        for i in 0..PICKS+1 {
            println!("Matched {}: {}", i, matches[i]);
            match_sum += matches[i];
            
            value = Self::get_value(i);
            if value == u32::MAX && matches[i] > 0 {
                win = u32::MAX;
                break;
            }
            else {
                win += matches[i] * value;
            }
        }
    
        if win == u32::MAX {
            println!("\nYou won it all, baby. 'grats")
        }
        else {
            println!("\nYou won a total of ${}", win);
        }
    
        assert_eq!(DRAWS as u32, match_sum);
    
        match Self::file_output(winners, draws, win) {
            Err(e) => println!("{:?}", e),
            _ => ()
        }
    }
    
    fn get_value(hits: usize) -> u32 {
        return match hits {
            0 | 1 => 0,
            2 => 1,
            3 => 5,
            4 => 200,
            _ => u32::MAX
        }
    }
    
    fn file_output(winners: [u8; PICKS], draws: Vec<Vec<u8>>, win:u32) -> std::io::Result<()> {
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
        output += "\n\n";
    
        output += "You won a total of ";
        output += &win.to_string();
        
        file.write_all(output.as_bytes())?;
    
        Ok(())
    }
}