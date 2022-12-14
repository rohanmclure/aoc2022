#![allow(non_snake_case)]

use std::env;
use std::fs;
use std::io;
use std::io::BufReader;

use aoc::parser::parse_non_empty_line;


#[derive(Clone, PartialEq, Eq, Debug)]
enum Packet {
    Sub(Vec<Packet>),
    Entry(usize)
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Packet::Sub(v) => {
                match rhs {
                    Packet::Sub(u) => {
                        for (i, e) in v.iter().enumerate() {
                            if i == u.len() {
                                return Some(std::cmp::Ordering::Greater);
                            }
                            let f = &u[i];

                            if e.gt(f) {
                                return Some(std::cmp::Ordering::Greater);
                            } else if e.lt(f) {
                                return Some(std::cmp::Ordering::Less)
                            }
                        }

                        if v.len() < u.len() {
                            Some(std::cmp::Ordering::Less)
                        } else {
                            Some(std::cmp::Ordering::Equal)
                        }
                    },
                    Packet::Entry(f) => {
                        self.partial_cmp(&Packet::Sub(vec![Packet::Entry(*f)]))
                    },
                }
            },
            Packet::Entry(e) => {
                match rhs {
                    Packet::Sub(_) => {
                        Packet::Sub(vec![Packet::Entry(*e)]).partial_cmp(rhs)
                    },
                    Packet::Entry(f) => {
                        e.partial_cmp(f)
                    }
                }
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.partial_cmp(rhs).unwrap()
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
                    subs.push(recurse(idx, s));
                },
                ']' => {
                    *idx += 1;
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

        if lp <= rp {
            sum += idx;
        }
    }

    println!("Part one: index sum of in-order packet pairs: {sum}");

    let mut packets = vec![];
    for mut pair in packet_pairs {
        packets.append(&mut pair);
    }

    /* Include dividers */
    let divider_A = Packet::Sub(vec![Packet::Sub(vec![Packet::Entry(2)])]);
    let divider_B = Packet::Sub(vec![Packet::Sub(vec![Packet::Entry(6)])]);
    packets.push(divider_A.clone());
    packets.push(divider_B.clone());

    packets.sort();

    let off_A = 1 + packets.binary_search(&divider_A).unwrap();
    let off_B = 1 + packets.binary_search(&divider_B).unwrap();

    println!("Part two: decoder key: {}", off_A * off_B);

    Ok(())
}