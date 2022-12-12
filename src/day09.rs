use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufReader;

use aoc::parser::parse_non_empty_line;

#[derive(Copy, Clone,Debug)]
enum Direction {
    Left,
    Right,
    Down,
    Up
}

impl Direction {
    fn move_(&self, (x, y): &mut (isize, isize)) {
        match self {
            Direction::Left => *x -= 1,
            Direction::Right => *x += 1,
            Direction::Down => *y -= 1,
            Direction::Up => *y += 1
        };
    }
}

/* Provides a metric on the rope-norm space, defined as the infima of
 * 8-directional movements.
 */
fn rope_norm(d: (isize, isize)) -> usize {
    let (x,y) = (d.0.abs() as usize,
                 d.1.abs() as usize);
    x.max(y)
}

fn simulate_rope(num_links: usize, moves: &Vec<Direction>) -> usize {

    if num_links < 2 {
        panic!("Don't use this method to model a single link.");
    }

    let mut links = vec![];
    links.resize(num_links, (0 as isize, 0 as isize));

    let mut tail_positions = HashSet::new();
    tail_positions.insert(*links.last().unwrap());

    for m in moves {
        m.move_(&mut links[0]);

        for i in 1 .. num_links {

            let rel_head = links[i-1];
            let rel_tail = &mut links[i];

            let diff = (rel_head.0 - rel_tail.0,
                        rel_head.1 - rel_tail.1);

            if rope_norm(diff) > 2 {
                panic!("Difference is too large");
            }

            if rope_norm(diff) < 2 {
                continue;
            }

            if diff.0 < 0 {
                Direction::Left.move_(rel_tail);
            } else if diff.0 > 0 {
                Direction::Right.move_(rel_tail);
            }
            if diff.1 < 0 {
                Direction::Down.move_(rel_tail);
            } else if diff.1 > 0 {
                Direction::Up.move_(rel_tail);
            }
        }

        tail_positions.insert(*links.last().unwrap());
    }

    tail_positions.len()
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

    let mut head_directions = vec![];
    while let Some(line) = parse_non_empty_line(&mut r) {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        let (d, n) = (parts[0].to_string(),
                      parts[1].to_string().parse::<usize>().unwrap());

        let d = match d.as_str() {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "U" => Direction::Up,
             _  => panic!("Invalid direction")
        };

        for _ in 0 .. n {
            head_directions.push(d);
        }
    }

    println!("Part one: tail positions reached: {}",
             simulate_rope(2, &head_directions));

    println!("Part two: tail positions reached: {}",
             simulate_rope(10, &head_directions));

    Ok(())
}