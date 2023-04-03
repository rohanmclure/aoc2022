#![feature(let_chains)]

use std::env;
use std::fs;
use std::io;
use std::io::BufReader;

use lazy_static::lazy_static;

use aoc::matrix::Matrix;
use aoc::parser::parse_non_empty_line;

lazy_static! {
    static ref START_POINT: (isize, isize) = (500,0);
}

#[derive(Clone, Copy, PartialEq)]
enum Element {
    Stone,
    Sand,
    Air
}

impl Element {
    fn blocks(&self) -> bool {
        match self {
            Element::Stone => true,
            Element::Sand  => true,
            Element::Air   => false,
        }
    }
}

fn parse_draw_directives(line: &String) -> Vec<(isize, isize)> {
    let mut v = vec![];
    for pair in line.split(" -> ") {
        let mut split = pair.splitn(2, ',');
        v.push((split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap()));
    }
    v
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

    /* Calculate relevant indices */
    let mut shapes = vec![];
    while let Some(line) = parse_non_empty_line(&mut r) {
        shapes.push(parse_draw_directives(&line));
    }

    let mut endpoints = vec![];
    for shape in shapes.iter() {
        for endpoint in shape.iter() {
            endpoints.push(endpoint.clone());
        }
    }

    /*
     *  x_range should be centered around the start point's x value
     *  y_range is +1 because I don't intend to represent the bottom.
     */
    let y_range = START_POINT.1 ..= (endpoints.iter().map(|e| e.1).max().unwrap() + 1);
    let x_range = {
        let diff = endpoints.iter().map(|e| e.0).max().unwrap()
                   - endpoints.iter().map(|e| e.0).min().unwrap() + 1;
        let len = diff.max(y_range.clone().count() as isize) * 2;
        (START_POINT.0 - len) ..= (START_POINT.0 + len)
    };

    let mut grid = Matrix::new(x_range.clone().count(),
                               y_range.clone().count());
    grid.fill(Element::Air);

    /* Draw the shapes */
    let as_idx = |(x,y)| {
        if  !x_range.contains(&x)
         || !y_range.contains(&y) {
            None
        } else {
            Some(((x - x_range.start()) as usize,
                  (y - y_range.start()) as usize))
        }
    };

    for shape in shapes.iter() {
        let (mut x, mut y) = shape[0].clone();
        grid[as_idx((x,y)).unwrap()] = Element::Stone;
        for endpoint in shape[1..].iter() {
            let diff = (endpoint.0 - x,
                        endpoint.1 - y);

            if diff.0 > 0 {
                for x in x+1 ..= endpoint.0 {
                    grid[as_idx((x,y)).unwrap()] = Element::Stone;
                }
            } else if diff.1 > 0 {
                for y in y+1 ..= endpoint.1 {
                    grid[as_idx((x,y)).unwrap()] = Element::Stone;
                }
            } else if diff.0 < 0 {
                for x in endpoint.0 ..= x-1 {
                    grid[as_idx((x,y)).unwrap()] = Element::Stone;
                }
            } else if diff.1 < 0 {
                for y in endpoint.1 ..= y-1 {
                    grid[as_idx((x,y)).unwrap()] = Element::Stone;
                }
            }

            (x, y) = endpoint.clone();
        }
    }

    /* Simulate */
    assert!(x_range.contains(&START_POINT.0));

    'grains: loop {
        let mut c_pos = *START_POINT;

        'simulate: loop {

            if *y_range.end() < c_pos.1 {
                /* We've entered and then exited the window. */
                break 'grains;
            }

            for next_pos in vec![(c_pos.0,     c_pos.1 + 1),
                                 (c_pos.0 - 1, c_pos.1 + 1),
                                 (c_pos.0 + 1, c_pos.1 + 1)] {

                if as_idx(next_pos).map_or(true, |idx| !grid[idx].blocks()) {
                    c_pos = next_pos;
                    continue 'simulate;
                }
            }

            /* No valid transitions remain. */
            grid[as_idx(c_pos).unwrap()] = Element::Sand;
            break 'simulate;
        }
    }

    let (m, n) = grid.get_dims();
    let mut num_sand = 0;
    for x in 0..m {
        for y in 0..n {
            if matches!(grid[(x,y)], Element::Sand) {
                num_sand += 1;
            }
        }
    }

    println!("Part one: final number of sand grains: {}", num_sand);

    grid.fill(Element::Air);

    'grains: loop {
        let mut c_pos = *START_POINT;

        if grid[as_idx(c_pos).unwrap()] == Element::Sand {
            break 'grains;
        }

        'simulate: loop {
            for next_pos in vec![(c_pos.0,     c_pos.1 + 1),
                                 (c_pos.0 - 1, c_pos.1 + 1),
                                 (c_pos.0 + 1, c_pos.1 + 1)] {

                /* Assume x range sufficiently large
                 * so that out of bounds --> we've hit
                 * bottom boundary
                 */
                if as_idx(next_pos).map_or(false,
                                           |idx| !grid[idx].blocks()) {

                    c_pos = next_pos;
                    continue 'simulate;
                }
            }

            /* No viable positions. */
            grid[as_idx(c_pos).unwrap()] = Element::Sand;
            break 'simulate;
        }
    }

    let (m, n) = grid.get_dims();
    let mut num_sand = 0;
    for x in 0..m {
        for y in 0..n {
            if matches!(grid[(x,y)], Element::Sand) {
                num_sand += 1;
            }
        }
    }

    println!("Part two: final number of sand grains: {}", num_sand);

    Ok(())
}