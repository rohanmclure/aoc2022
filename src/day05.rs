use std::io::{BufReader, self};
use std::fs::File;
use std::env;

use lazy_static::lazy_static;
use regex::Regex;

use aoc::parser::parse_non_empty_line;

lazy_static! {
    static ref DIRECTIVE_REGEX: Regex = Regex::new(r"move ([0-9]+) from ([0-9]) to ([0-9])")
                                              .unwrap();
    static ref STACKS_INIT: Vec<Vec<char>> = vec![
    vec!['W','B','D','N','C','F','J'],
    vec!['P', 'Z', 'V', 'Q', 'L', 'S', 'T'],
    vec!['P', 'Z', 'B', 'G', 'J', 'T'],
    vec!['D', 'T', 'L', 'J', 'Z', 'B', 'H', 'C'],
    vec!['G', 'V', 'B', 'J', 'S'],
    vec!['P', 'S', 'Q'],
    vec!['B', 'V', 'D', 'F', 'L', 'M', 'P', 'N'],
    vec!['P', 'S', 'M', 'F', 'B', 'D', 'L', 'R'],
    vec!['V', 'D', 'T', 'R']
  /*                [C]         [N] [R]    
   *    [J] [T]     [H]         [P] [L]    
   *    [F] [S] [T] [B]         [M] [D]    
   *    [C] [L] [J] [Z] [S]     [L] [B]    
   *    [N] [Q] [G] [J] [J]     [F] [F] [R]
   *    [D] [V] [B] [L] [B] [Q] [D] [M] [T]
   *    [B] [Z] [Z] [T] [V] [S] [V] [S] [D]
   *    [W] [P] [P] [D] [G] [P] [B] [P] [V]
   *    1   2   3   4   5   6   7   8   9 
   */
    ];
}

// struct Directive {
//     amount: usize,
//     source_stack: usize,
//     dest_stack: usize
// }

// impl Directive {
//     fn new(amount: usize, source: usize, dest: usize) -> Directive {
//         Directive { amount: amount, source_stack: source, dest_stack: dest }
//     }
// }

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of args!")
    }

    let mut r = {
        let f = File::open(&args[1])?;
        BufReader::new(f)
    };
    
    // Clone our stacks
    let mut stacks_p1 = STACKS_INIT.clone();
    let mut stacks_p2 = STACKS_INIT.clone();

    while let Some(line) = parse_non_empty_line(&mut r) {
        let (n, s, d) = {
            let captures = DIRECTIVE_REGEX.captures(&line).unwrap();
            (captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
             captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
             captures.get(3).unwrap().as_str().parse::<usize>().unwrap())
        };
        
        /* Part One manipulations */
        let mut v = vec![];
        {
            let source = &mut stacks_p1[s-1];
            for _ in 0..n {
                v.push(source.pop().unwrap());            
            }
        }
        {
            let dest = &mut stacks_p1[d-1];
            dest.append(&mut v);
        }

        /* Part Two manipulations */
        let mut v = vec![];
        {
            let source = &mut stacks_p2[s-1];
            for i in source.len()-n .. source.len() {
                v.push(source[i]);
            }
            source.resize(source.len()-n, char::MAX);
        }
        {
            let dest = &mut stacks_p2[d-1];
            dest.append(&mut v);
        }
    }
    
    let mut tops_p1 = vec![];
    let mut tops_p2 = vec![];
    for stack in stacks_p1.iter() {
        tops_p1.push(stack.last().unwrap().clone());
    }
    for stack in stacks_p2.iter() {
        tops_p2.push(stack.last().unwrap().clone());
    }
    
    println!("The tops of the stacks spell: {:?}", tops_p1);
    println!("The tops of the stacks spell: {:?}", tops_p2);

    Ok(())
}