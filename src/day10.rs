#![allow(non_snake_case)]
use std::env;
use std::fs;
use std::io;
use std::io::BufReader;

use lazy_static::lazy_static;
use regex::Regex;

use aoc::parser::parse_non_empty_line;
use aoc::matrix::Matrix;


lazy_static! {
    static ref OP_REGEX: Regex
         = Regex::new(r"(noop)|(addx (-?[0-9]+))")
                 .unwrap();
    static ref REG_X_START: isize = 1;
}

/* Simple state machine with registers */
struct Core {
    rX: isize,
    cycle: usize,
    in_flight: Option<Op>,
    local_cycle: usize
}

impl Core {
    fn new(init_rX: isize) -> Self {
        Core { rX: init_rX, cycle: 0, in_flight: None, local_cycle: 0 }
    }

    fn issue(&mut self, instr: Op) {
        self.in_flight = Some(instr);
    }

    fn dispatch(&mut self) -> bool {
        let mut completed = false;

        self.local_cycle += 1;
        match self.in_flight.unwrap_or_else(||
                    panic!("Dispatch with no instructions in queue")) {
            Op::Noop => {
                self.commit();
                completed = true;
            },
            Op::Addx(imm) => {
                if self.local_cycle == 2 {
                    self.rX += imm;
                    self.commit();
                    completed = true;
                }
            }
        }
        completed
    }

    fn advance_clock(&mut self) {
        self.cycle += 1;
    }

    fn commit(&mut self) {
        self.local_cycle = 0;
        self.in_flight = None;
    }

    fn get_cycle(&self) -> usize {
        self.cycle
    }

    fn get_regX(&self) -> isize {
        self.rX
    }
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Noop,
    Addx(isize)
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

    let mut ops = vec![];
    while let Some(line) = parse_non_empty_line(&mut r) {
        let captures = OP_REGEX.captures(&line).unwrap();

        if let Some(_noop) = captures.get(1) {
            ops.push(Op::Noop);
        } else if let Some(_addx) = captures.get(2) {
            let imm = captures.get(3).unwrap()
                                     .as_str()
                                     .parse::<isize>()
                                     .unwrap();
            ops.push(Op::Addx(imm));
        }
    }

    /* Part one no pipeline. */
    let mut cpu = Core::new(*REG_X_START);
    let mut signal_strengths = vec![];
    for op in ops.iter() {
        cpu.issue(*op);
        while {
            cpu.advance_clock();
            let c = cpu.get_cycle();
            if vec![20,60,100,140,180,220].contains(&c) {
                println!("{c}: {}, {:?}", (c as isize) * cpu.get_regX(), cpu.in_flight);
                signal_strengths.push((c as isize) * cpu.get_regX());
            }

            !cpu.dispatch()
        } {}

    }

    println!("Part one: signal strength: {}", signal_strengths.iter().sum::<isize>());

    let mut crt = Matrix::new(40, 6);

    let mut cpu = Core::new(*REG_X_START);
    let mut sprite_range = {
        let rX = cpu.get_regX();
        ((rX-1) as usize) ..=((rX+1) as usize)
    };
    for op in ops.iter() {
        cpu.issue(*op);

        while {
            cpu.advance_clock();
            let c = cpu.get_cycle();
            let x = (c-1) % 40;
            let y = (c-1) / 40;

            crt[(x,y)] = sprite_range.contains(&x);

            !cpu.dispatch()
        } {}

        sprite_range = {
            let rX = cpu.get_regX();
            ((rX-1) as usize) ..=((rX+1) as usize)
        };
    }

    println!("Part two:");
    for y in 0 .. 6 {
        for x in 0 .. 40 {
            print!("{}", if crt[(x,y)] { '#' } else { '.' });
        }
        println!();
    }

    Ok(())
}