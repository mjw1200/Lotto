use std::io::{self, Write};
use rand::{thread_rng, Rng};
use std::fs::File;

fn main() {
    print!("Generate (g) or check (c)? ");
    io::stdout().flush().unwrap();
    
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer).unwrap();

    if buffer.starts_with("g") {
        generate()
    }
    else if buffer.starts_with("c") {
        check()
    }
}

fn generate() {
    let mut rng = thread_rng();
    let mut picks: [u8; 5] = [0; 5];
    let mut outfile = File::create("foo.txt").unwrap();

    for _ in 0..10 {
        for j in 0..5 {
            picks[j] = rng.gen_range(1..45);  
        }
        picks.sort(); 
        output(&mut outfile, picks);     
        outfile.write(b"\n").unwrap();
    }

    outfile.flush().unwrap();
}

fn output(of: &mut File, picks : [u8; 5]) {
    for i in 0..5 {
        print!("{} ", picks[i]);
        write!(of, "{} ", picks[i]).unwrap();
    }
    println!();
}

fn check() {
    println!("Check")
}
