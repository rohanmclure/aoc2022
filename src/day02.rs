use std::io::{BufReader, BufRead, self};
use std::fs::File;
use std::error;
use std::env;


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;


#[derive(PartialEq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors
}

impl Play {
    pub fn score(&self) -> usize {
        match *self {
            Rock        => 1,
            Paper       => 2,
            Scissors    => 3
        }
    }
    
    pub fn verse(&self, other: &Play) -> Outcome {

        if *self == *other {
            return Outcome::Draw
        }
        
        

        unimplemented!()
    }
}


#[derive(Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win
}

impl Outcome {
    pub fn score(&self) -> usize {
        match *self {
            Loss    => 0,
            Draw    => 3,
            Win     => 6
        }
    }
}

struct Game {
    us: Play,
    them: Play
}

impl Game {
    fn outcome(&self) -> Outcome {
        self.us.verse(&self.them)
    }
    
    pub fn score(&self) -> usize {
        self.outcome()
            .score()
         + self.us.score()
    }
}


fn parse(r: &mut BufReader<File>) -> Result<Vec<Game>> {
    let v = vec![];
    
    Ok(v)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of args!")
    }

    let games = {
        let f = File::open(&args[1])?;
        parse(&mut BufReader::new(f))
             .unwrap_or_else(|_| panic!("Unable to parse file"))
    };
    
    let total_score = games.iter()
                           .map(|g| g.score())
                           .sum::<usize>();
    
    println!("The total score under the strategy is {total_score}");

    Ok(())
}