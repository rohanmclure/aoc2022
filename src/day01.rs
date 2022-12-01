use std::io::{BufReader, BufRead, self, Read};
use std::fs::File;
use std::str::FromStr;
use std::error;
use std::env;


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn parse(r: &mut BufReader<File>) -> Result<Vec<Vec<u64>>> {
    let mut elves = vec![];

    let mut line = String::from_str("").unwrap();
    loop {
        let mut v: Vec<u64> = vec![];
        loop {
            r.read_line(&mut line)?;

            if line.is_empty() {
                break;
            }

            v.push(line.parse::<u64>()?);
        }

        elves.push(v);
    }
    Ok(elves)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of args!")
    }

    let f = File::open(&args[1])?;

    let v = parse(&mut BufReader::new(f));

    let mut max: u64 = 0;
    let mut max_i: u64 = 0;

    for (i, elf) in v.iter().enumerate() {
        let local_max = elf.iter().max().unwrap_or_else(|_| 0);
        if local_max > max {
            max = local_max;
        }
    }

    Ok(())
}
