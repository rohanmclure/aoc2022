use std::io::{BufReader, BufRead, self};
use std::fs::File;
use std::error;
use std::env;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;


#[derive(PartialEq, Clone, Copy, Debug)]
enum Play {
    Rock        = 0,
    Paper       = 1,
    Scissors    = 2
}

impl Play {
    pub fn score(&self) -> usize {
        match *self {
            Play::Rock        => 1,
            Play::Paper       => 2,
            Play::Scissors    => 3
        }
    }
    
    pub fn verse(&self, other: &Play) -> Outcome {

        let diff = {
            let d = (*self as i8 - *other as i8) % 3;
            (d + 3) % 3
        };
        
        match diff {
            0 => Outcome::Draw,
            1 => Outcome::Win,
            2 => Outcome::Loss, /* 2 = -1 mod 3 */
            _ => unreachable!()
        }
    }

    pub fn from_code(c: char) -> Option<Play> {
        match c {
            'A' => Some(Play::Rock),
            'X' => Some(Play::Rock),

            'B' => Some(Play::Paper),
            'Y' => Some(Play::Paper),

            'C' => Some(Play::Scissors),
            'Z' => Some(Play::Scissors),

            _   => None
        }
    }
    
    pub fn reinterpet_as_outcome(&self) -> Outcome {
        match *self {
            Play::Rock      => Outcome::Loss,
            Play::Paper     => Outcome::Draw,
            Play::Scissors  => Outcome::Win
        }
    }
}


#[derive(Clone, Copy)]
enum Outcome {
    Loss = 2 /* -1 mod 3 */,
    Draw = 0,
    Win  = 1
}

impl Outcome {
    pub fn score(&self) -> usize {
        match *self {
            Outcome::Loss    => 0,
            Outcome::Draw    => 3,
            Outcome::Win     => 6
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
    
    pub fn score(&self, reinterpret: bool) -> usize {

        if !reinterpret {
            self.outcome()
                .score()
             + self.us.score()
        } else {
            let desired_outcome = self.us.reinterpet_as_outcome();
            desired_outcome.score()
             + self.force_outcome_play(desired_outcome)
                   .score()
        }
    }
    
    pub fn force_outcome_play(&self, outcome: Outcome) -> Play {
        let p = self.them;
        match (p as i8 + outcome as i8) % 3 { 
            0   => Play::Rock,
            1   => Play::Paper,
            2   => Play::Scissors,
            _   => unreachable!()
        }
    }
    
    pub fn new(us: Play, them: Play) -> Game {
        Game { us, them }
    }
}


fn parse(r: &mut BufReader<File>) -> Result<Vec<Game>> {
    let mut v = vec![];

    'outer: loop {
        let line;

        /* Get non-empty payload line */
        loop {
            line = {
                let mut l = String::new();
                let bytes = r.read_line(&mut l)?;
                let l_prefix = l.trim_end();
                if bytes == 0 {
                    break 'outer;
                }
                if l_prefix.is_empty() {
                    continue;
                }
                String::from_str(l_prefix)?
            };
            break;
        }
        
        /* Regex parse the game */
        let (them, us) = {
            let caps = GAME_REGEX.captures(line.as_str()).unwrap();
            (caps.get(1).unwrap()
                        .as_str()
                        .parse::<char>()?,
             caps.get(2).unwrap()
                        .as_str()
                        .parse::<char>()?)
        };

        v.push(Game::new(
               Play::from_code(us).unwrap(),
               Play::from_code(them).unwrap()
               ));
    }

    Ok(v)
}

lazy_static! {
    static ref GAME_REGEX: Regex = Regex::new(r"([A-C])\s+([X-Z])").unwrap();
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
    
    /* Interpret right column as suggested response */
    let total_score = games.iter()
                           .map(|g| g.score(false))
                           .sum::<usize>();

    println!("The total score under the strategy is {total_score}");
    
    /* Interpret right column as desired win state */
    let total_score = games.iter()
                           .map(|g| g.score(true))
                           .sum::<usize>();

    println!("The total score under the strategy is {total_score}");

    Ok(())
}