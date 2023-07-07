use crate::PICKS;
use std::fs::File;
use std::io::Write;

pub struct Utils {
}

impl Utils {
    pub fn get_value(hits: usize) -> u32 {
        return match hits {
            0 | 1 => 0,
            2 => 1,
            3 => 5,
            4 => 200,
            _ => u32::MAX
        }
    }
    
    pub fn file_output(winners: [u8; PICKS], draws: Vec<Vec<u8>>, win:u32) -> std::io::Result<()> {
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