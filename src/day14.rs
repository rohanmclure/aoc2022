#![feature(let_chains)]

use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Index;
use std::ops::IndexMut;

use lazy_static::lazy_static;
use regex::Regex;

use aoc::matrix::Matrix;
use aoc::parser::parse_non_empty_line;

lazy_static! {
    static ref POINT_REGEX: Regex = Regex::new(r"(\-?[0-9]+),(\-?[0-9])+")
                          .unwrap();
}

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn straight_line(&self, p: &Self) -> Option<Box<dyn Iterator<Item=Self>>> {
        let q = self;

        if (q.x != p.x) && (q.y != p.y) {
            return None;
        }

        if q.x == p.x {
            let x = q.x;
            let a = p.y.min(q.y);
            let b = p.y.max(q.y);
            Some(Box::new((a..=b).map(move |y| Point { x, y })))
        } else {
            let y = q.y;
            let a = p.x.min(q.x);
            let b = p.x.max(q.x);
            Some(Box::new((a..=b).map(move |x| Point { x, y })))
        }
    }

    fn to_relative(self, p: Self) -> Self {
        Point {
            x: self.x - p.x,
            y: self.y - p.y
        }
    }

    fn from_relative(self, p: Self) -> Self {
        Point {
            x: self.x + p.x,
            y: self.y + p.y
        }
    }
}

impl<T> Index<Point> for Matrix<T> {
    type Output = T;

    fn index(&self, Point { x, y }: Point) -> &Self::Output {
        &self[(x as usize, y as usize)]
    }
}

impl<T> IndexMut<Point> for Matrix<T> {
    fn index_mut(&mut self, Point { x, y }: Point) -> &mut Self::Output {
        &mut self[(x as usize, y as usize)]
    }
}

#[derive(Clone, Copy)]
enum Element {
    Rock,
    Sand,
    Air
}

impl Element {
    fn is_rigid(&self) -> bool {
        match self {
            Self::Rock => true,
            Self::Sand => true,
            Self::Air  => false
        }
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let f = fs::File::open(&args[1]).unwrap();
    let mut r = BufReader::new(f);

    let mut line = String::new();
    let mut points = vec![];
    while let Ok(_) = r.read_line(&mut line) {

        let mut pairs = vec![];
        for e in line.split("->") {
            let cap = POINT_REGEX.captures(e.trim()).unwrap();
            pairs.push(Point { x: i64::from_str_radix(cap.get(1).unwrap().as_str(), 10).unwrap(),
                               y: i64::from_str_radix(cap.get(2).unwrap().as_str(), 10).unwrap()} );
        }

        for i in 0 .. pairs.len() - 1 {
            points.extend(pairs[i].straight_line(&pairs[i+1])
                                  .unwrap());
        }

    }

    /* Get the maximum, minimum of all coordinates */
    let x_range = (
        points.iter().map(|p| p.x).min().unwrap(),
        points.iter().map(|p| p.x).max().unwrap()
    );

    let y_range = (
        points.iter().map(|p| p.y).min().unwrap(),
        points.iter().map(|p| p.y).max().unwrap()
    );

    let (m, n) = (
        x_range.1 - x_range.0 + 1,
        y_range.1 - y_range.0 + 1
    );

    let r = Point { x: x_range.0, y: y_range.1 };
    let mut g: Matrix<Element> = Matrix::new(m as usize, n as usize);
    g.fill(Element::Air);

    {
        let points: Vec<Point> = points.iter().map(|p| p.to_relative(r)).collect();
        for p in points {
            g[p] = Element::Rock;
        }
    }

    /*
     * Base problem:
     */

    loop {
        let mut s = Point { x: 500, y: 0 }.to_relative(r);
        loop {
            if g[s].is_rigid() {
                panic!("Invalid start position for sand.");
            }

            /* figure out where to go */
            let n = Point { x: s.x, y: s.y - 1 };
            if !g[n].is_rigid() {
                s = n;
                continue;
            }

            let l = Point { x: s.x - 1, y: s.y - 1 };
            let r = Point { x: s.x + 1, y: s.y - 1 };
            if !g[l].is_rigid() {
                s = l;
                continue;
            } else if !g[r].is_rigid() {
                s = r;
                continue;
            }

            break;
        }
        g[s] = Element::Sand;
    }
    
}
