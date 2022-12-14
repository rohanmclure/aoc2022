
use std::env;
use std::fs;
use std::io;
use std::io::BufReader;

use aoc::parser::parse_non_empty_line;


#[derive(Debug)]
enum Packet {
    Sub(Vec<Packet>),
    Entry(usize)
}

impl Packet {
    fn le(&self, rhs: &Packet) -> bool {
        match self {
            Packet::Sub(v) => {
                match rhs {
                    Packet::Sub(u) => {
                        for (i, e) in v.iter().enumerate() {
                            if i == u.len() {
                                println!("Comparing {:?}, {:?}: false", self, rhs);
                                return false;
                            }
                            let f = &u[i];
                            if !e.le(f) {
                                println!("Comparing {:?}, {:?}: false", self, rhs);
                                return false;
                            }
                        }
                        println!("Comparing {:?}, {:?}: true", self, rhs);
                        true
                    },
                    Packet::Entry(f) => {
                        let b = self.le(&Packet::Sub(vec![Packet::Entry(*f)]));
                        println!("Comparing {:?}, {:?}: {}", self, rhs, b);
                        b
                    },
                }
            },
            Packet::Entry(e) => {
                match rhs {
                    Packet::Sub(_) => {
                        let b = Packet::Sub(vec![Packet::Entry(*e)]).le(rhs);
                        println!("Comparing {:?}, {:?}: {}", self, rhs, b);
                        b
                    },
                    Packet::Entry(f) => {
                        let b = *e <= *f;
                        println!("Comparing {:?}, {:?}: {}", self, rhs, b);
                        b
                    }
                }
            }
        }
    }
}

fn parse_packet(line: &String) -> Packet {

    let s: Vec<char> = line.as_str().chars().collect();
    let mut idx = 0;

    fn recurse(idx: &mut usize, s: &Vec<char>) -> Packet {
        assert!(s[*idx] == '[');
        *idx += 1; /* consume opening bracket */

        let mut subs = vec![];
        while *idx != s.len() {
            let c = s[*idx];
            match c {
                '0'..='9' => {
                    let mut consecutive_numbers = vec![];
                    while {
                        let num_start = *idx;
                        loop {
                            let numeric = ('0'..='9').contains(&s[*idx]);
                            if !numeric {
                                break;
                            }
                            *idx += 1;
                        }
                        consecutive_numbers.push(String::from_iter(s[num_start..*idx].iter())
                                                 .parse::<usize>().unwrap());
                        // println!("Parsed number: {}", consecutive_numbers.last().unwrap());

                        match s[*idx] {
                            ',' => {
                                *idx += 1; /* assume no whitespace */
                            }
                             _  => {}
                        }

                        match s[*idx] {
                            '0'..='9' => true,
                            _         => false
                        }
                    } {}
                    subs.append(&mut consecutive_numbers.iter().map(|n| Packet::Entry(*n)).collect());
                },
                '[' => {
                    // println!("Recursing with remainder: {}", String::from_iter(s[*idx..].iter()));
                    subs.push(recurse(idx, s));
                },
                ']' => {
                    *idx += 1;
                    // println!("Returning to parse remainder: {}", String::from_iter(s[*idx..].iter()));
                    return Packet::Sub(subs);
                },
                ',' => {
                    *idx += 1;
                },
                 _  => {
                    panic!("Invalid character '{c}' at idx {}", *idx);
                }
            }
        }

        panic!("Failed to parse subpacket");
    }

    recurse(&mut idx, &s)
}

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of args!")
    }

    let mut r = {
        let f = fs::File::open(&args[1])?;
        BufReader::new(f)
    };

    let mut v = vec![];
    while let Some(line) = parse_non_empty_line(&mut r) {
        v.push(line);
    }

    let mut packet_pairs = vec![];
    for p in 0 .. v.len() / 2 {
        let mut pair = vec![];
        for e in 0 .. 2 {
            let packet_txt = &v[p*2 + e];
            pair.push(parse_packet(packet_txt));
        }
        packet_pairs.push(pair);
    }

    let mut sum = 0;
    for (i, pair) in packet_pairs.iter().enumerate() {

        let idx = i+1;
        let (lp, rp) = (&pair[0], &pair[1]);

        if lp.le(&rp) {
            sum += idx;
        }
    }

    println!("Part one: index sum of in-order packet pairs: {sum}");

    Ok(())
}