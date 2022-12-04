use std::collections::HashSet;
use std::hash::Hash;
use std::io::{BufReader, BufRead, self};
use std::fs::File;
use std::error;
use std::env;
use std::str::FromStr;
use std::string::ParseError;

use lazy_static::lazy_static;
use regex::Regex;


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

lazy_static! {
    static ref RUCKSACK_REGEX: Regex = Regex::new(r"[a-zA-Z]+").unwrap();
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Item {
    code: u8
}

impl Item {
    pub fn from_code(code: u8) -> Item {
        Item { code: code }
    }
    
    pub fn get_priority(&self) -> usize {
        let code = self.code;
        if 'a' as u8 <= code && code <= 'z' as u8 {
            1 + (code - 'a' as u8) as usize
        } else if 'A' as u8 <= code && code <= 'Z' as u8 {
            27 + (code - 'A' as u8) as usize
        } else {
            unreachable!()
        }
    }
}

struct Group {
    rs: (Rucksack, Rucksack, Rucksack)
}

impl Group {
    pub fn from_rucksacks(r1: Rucksack,
                          r2: Rucksack,
                          r3: Rucksack) -> Group {
        Group { rs: (r1, r2, r3) }
    }

    pub fn get_badge(&self) -> Item {
        let (r1, r2, r3) = &self.rs;

        let (v1, v2, v3) = (r1.get_all(),
                            r2.get_all(),
                            r3.get_all());
        
        for e1 in v1.iter() {
            for e2 in v2.iter() {
                if *e1 != *e2 {
                    continue;
                }

                for e3 in v3.iter() {
                    if *e1 == *e3 {
                        return *e1;
                    }
                }
            }
        }
        
        println!("{:?}", v1);
        println!("{:?}", v2);
        println!("{:?}", v3);
        
        unreachable!()
    }
}

#[derive(Clone)]
struct Rucksack {
    c1: Vec<Item>,
    c2: Vec<Item>,
}

impl Rucksack {
    pub fn from_compontent_strings(l: &str, r: &str) -> Rucksack {
        let v1 = l.as_bytes().iter()
                  .map(|&c| Item::from_code(c))
                  .collect();
        let v2 = r.as_bytes().iter()
                  .map(|&c| Item::from_code(c))
                  .collect();
        Rucksack { c1: v1, c2: v2 }
    }
    
    /* sillily assume get intersection will always return a singelton */
    pub fn get_intersection(&self) -> Vec<Item> {
        let mut s1 = HashSet::new();

        for e in self.c1.iter() {
            s1.insert(*e);
        }
        
        let mut s2 = HashSet::new();
        for e in self.c2.iter() {
            if s1.contains(e) {
                s2.insert(*e);
            }
        }

        s2.iter().map(|e| *e).collect()
    }
    
    pub fn get_all(&self) -> Vec<Item> {
        let mut v = self.c1.clone();
        v.append(&mut self.c2.clone());
        v
    }
}


fn parse(r: &mut BufReader<File>) -> Result<Vec<Group>> {

    let mut v = vec![];
    
    let mut parse_group = || -> Option<Group> {
        
        let mut rs = vec![];
        
        'outer: for _ in 0..3 {
            let line;

            /* Get non-empty payload line */
            loop {
                line = {
                    let mut l = String::new();
                    let bytes = r.read_line(&mut l).unwrap();
                    let l_prefix = l.trim_end();
                    if bytes == 0 {
                        return None;
                    }
                    if l_prefix.is_empty() {
                        continue;
                    }
                    String::from_str(l_prefix).unwrap()
                };
                break;
            }

            /* Regex parse the game */
            let rucksack_str = RUCKSACK_REGEX.captures(&line)
                                             .unwrap()
                                             .get(0)
                                             .unwrap()
                                             .as_str();

            let l = rucksack_str.len();
            if l % 2 != 0 {
                panic!("Malformed rucksack")            
            }
            
            rs.push(Rucksack::from_compontent_strings(
                &rucksack_str[0..l/2],
                &rucksack_str[l/2..l]
            ))
        }

        Some(Group::from_rucksacks(rs[0].clone(), rs[1].clone(), rs[2].clone()))
    };

    while let Some(g) = parse_group() {
        v.push(g)
    }

    Ok(v)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of args!")
    }
    
    let groups = {
        let f = File::open(&args[1])?;
        parse(&mut BufReader::new(f))
             .unwrap_or_else(|_| panic!("Unable to parse file"))
    };
    
    let priority_sum = groups.iter()
                             .map(|g| g.get_badge().get_priority())
                             .sum::<usize>();
    
    println!("Got the sum of priorities: {}", priority_sum);

    Ok(())
}