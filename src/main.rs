use std::io::{Write};
use rand::{thread_rng, Rng};
use std::fs::File;
use array2d::{Array2D};

fn main() {
    const PICKS: usize = 5; // Numbers in one draw. MT Cash = 5
    const DRAWS: usize = 5; // Number of draws

    let mut rng = thread_rng();
    let mut outfile = File::create("foo.txt").unwrap();
    
    let mut prefilled:Array2D<u8> = Array2D::filled_with(0, DRAWS, PICKS);

    for i in 0..DRAWS {
        for j in 0..PICKS {
            prefilled.set(i, j, rng.gen_range(1..45)).unwrap();
        }

        // let mut array: [u8; 5] = prefilled[i];

        // let mut vecvec = prefilled.as_rows();
        // vecvec[i].sort();
        // println!("{:?}", vecvec[i])
    }

    let mut rows = prefilled.as_rows();
    for i in 0..rows.len() {
        let _ = &rows[i].sort();
    }

    println!("{:?}", rows);

    // for _ in 0..10 {
    //     for j in 0..5 {
    //         picks[j] = rng.gen_range(1..45);  
    //     }
    //     picks.sort(); 
    //     output(&mut outfile, picks);     
    //     outfile.write(b"\n").unwrap();
    // }

    // outfile.flush().unwrap();
}

fn output(of: &mut File, picks : [u8; 5]) {
    for i in 0..5 {
        print!("{} ", picks[i]);
        write!(of, "{} ", picks[i]).unwrap();
    }
    println!();
}
