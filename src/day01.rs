use std::io::{BufReader, BufRead, self};
use std::fs::File;
use std::str::FromStr;
use std::error;
use std::env;


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn parse(r: &mut BufReader<File>) -> Result<Vec<Vec<usize>>> {
    let mut elves = vec![];

    'outer: loop {

        let mut v: Vec<usize> = vec![];
        loop {
            let mut l = String::from_str("").unwrap();

            let bytes = r.read_line(&mut l)?;
            let line = l.trim_end();
            println!("{line}");
            if bytes == 0 {
                break 'outer;                
            }

            if line.is_empty() {
                break;
            }

            v.push(line.parse::<usize>()?);
        }

        elves.push(v);
    }

    Ok(elves)
}

/* Dangerously inefficient. How am I a computer scientist? */
fn top_few(v: &Vec<usize>, n: usize) -> Vec<usize> {
    let mut v = v.clone();
    let mut ret = vec![];

    for _ in 0..n {
        let (i,_) = v.iter().enumerate()
                     .max_by(|(_,a), (_,b)| a.cmp(b)).unwrap();

        let m = v[i];
        v.remove(i);
        ret.push(m);
    }
    
    ret
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of args!")
    }

    let f = File::open(&args[1])?;

    let v = parse(&mut BufReader::new(f))
                .unwrap_or_else(|_| panic!("Unable to parse file"));
    
    let mut elf_sums: Vec<usize> = vec![];

    for elf in v {
        elf_sums.push(elf.iter().sum());
    }
    
    let top_elves = top_few(&elf_sums, 3);
    
    println!("The maximum caloric value is: {}",
             top_elves[0]);
    
    println!("Best three elves have total caloric intake: {}",
             top_elves.iter().sum::<usize>());

    Ok(())
}