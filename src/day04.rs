use std::io::{BufReader, self};
use std::fs::File;
use std::env;

use lazy_static::lazy_static;
use regex::Regex;

use aoc::parser::parse_non_empty_line;

lazy_static! {
    static ref ASSIGNMENT_REGEX: Regex = Regex::new(r"([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)").unwrap();
}

/* is rb a subset of ra ? */
fn range_subset(ra: &(usize, usize), rb: &(usize, usize)) -> bool {
    ra.0 <= rb.0 && rb.1 <= ra.1
}

/* do ra and rb overlap? (symetric) */
fn range_overlap(ra: &(usize, usize), rb: &(usize, usize)) -> bool {
    fn left_overlap(ra: &(usize, usize), rb: &(usize, usize)) -> bool {
        ra.0 <= rb.0 && rb.0 <= ra.1
    }

    range_subset(ra, rb) || range_subset(ra, rb)
                         || left_overlap(ra, rb)
                         || left_overlap(rb, ra)
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

    let mut line_no = 0;
    let mut subset_pairs = vec![];
    let mut overlap_pairs = vec![];
    while let Some(line) = parse_non_empty_line(&mut r) {

        let captures = ASSIGNMENT_REGEX.captures(&line).unwrap();

        let ra = (captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                  captures.get(2).unwrap().as_str().parse::<usize>().unwrap());
        let rb = (captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                  captures.get(4).unwrap().as_str().parse::<usize>().unwrap());

        if range_subset(&ra, &rb) || range_subset(&rb, &ra) {
            subset_pairs.push(line_no);
        }

        if range_overlap(&ra, &rb) {
            overlap_pairs.push(line_no);
        }

        line_no += 1;
    }

    println!("Number of subset pairs: {}", subset_pairs.len());
    println!("Number of overlap pairs: {}", overlap_pairs.len());

    Ok(())
}