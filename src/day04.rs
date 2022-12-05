use std::io::{BufReader, BufRead, self};
use std::fs::File;
use std::str::FromStr;
use std::error;
use std::env;

use aoc::matrix::Matrix;
use aoc::parser::parse_non_empty_line;

#[derive(Clone)]
struct Board {
    grid: Matrix<usize>
}

impl Board {
    fn new(grid: Matrix<usize>) -> Board {
        Board { grid: grid }
    }
}

/* Just panic on parse error */
fn parse_bingo_board(r: &mut BufReader<File>) -> Option<Board> {
    let mut grid = Matrix::new(5, 5);

    for i in 0..5 {
        if let Some(line) = parse_non_empty_line(r) {
            let split = line.splitn(5, ' ');
            for (j, s) in split.into_iter().enumerate() {
                grid[(i,j)] = s.to_string()
                               .parse::<usize>()
                               .unwrap();
            }
        }

        return None;
    }

    Some(Board::new(grid))
}

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of args!")
    }

    let r = {
        let f = File::open(&args[1])?;
        BufReader::new(f)
    };

    /* parse code */
    

    Ok(())
}