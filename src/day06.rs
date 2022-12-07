
use std::collections::VecDeque;
use std::io::{BufReader, self};
use std::fs::File;
use std::env;

use lazy_static::lazy_static;

use aoc::parser::parse_non_empty_line;

lazy_static! {
    static ref PACKET_LENGTH: usize = 14;
}

fn check_unique(v: &VecDeque<char>) -> bool {
    let l = v.len();
    for i in 0..l {
        for j in i+1..l {
            if v[i] == v[j] {
                return false;
            }
        }
    }
    true
}

fn main() -> io::Result<()> {
    
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of args!")
    }

    let mut r = {
        let f = File::open(&args[1])?;
        BufReader::new(f)
    };   
    
    let line = parse_non_empty_line(&mut r).unwrap();
    
    let marker = {
        let mut view = VecDeque::new();
        let mut idx = None;

        for (i, c) in line.as_bytes()
                          .iter()
                          .map(|c| *c as char)
                          .enumerate() {
            if view.len() == *PACKET_LENGTH {
                view.pop_front();
            }
            view.push_back(c);
            
            if  view.len() == *PACKET_LENGTH
             && check_unique(&view) {
                idx = Some(i + 1);
                println!("{:?}", view);
                break;
            }
        }
        
        idx.unwrap()
    };
    
    println!("First such marker found at {marker}");
    
    Ok(())
}