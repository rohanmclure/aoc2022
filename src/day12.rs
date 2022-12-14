#![feature(let_chains)]
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io;
use std::io::BufReader;
use std::mem;

use priority_queue::PriorityQueue;

use aoc::parser::parse_non_empty_line;
use aoc::matrix::Matrix;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of args!")
    }

    let mut r = {
        let f = fs::File::open(&args[1])?;
        BufReader::new(f)
    };

    let mut lines = vec![];
    while let Some(line) = parse_non_empty_line(&mut r) {
        lines.push(line);
    }

    let mut topo = Matrix::new(lines.len(),
                               lines[1].len());
    let mut start = (0,0);
    let mut end = (0,0);

    let (m, n) = topo.get_dims();
    for i in 0 .. m {
        let line = &lines[i];
        for j in 0 .. n {
            let c = line.get(j..=j).unwrap().as_bytes()[0];
            topo[(i,j)] = match c as char {
                'S' => {
                    start = (i,j);
                    1
                },
                'E' => {
                    end = (i,j);
                    26
                },
                 _  => c - ('a' as u8) + 1
            };
        }
    }

    /* breadth first-search */
    let mut dist = Matrix::new(m, n);
    dist.fill(0);
    let mut scheduled = Matrix::new(m, n);
    scheduled.fill(false);
    let mut visit_queue = VecDeque::new();

    let mut reached = 0;
    visit_queue.push_front(start);
    scheduled[start] = true;

    let neighbours = |(i,j)| {
        let mut v = vec![];
        if i > 0 {
            v.push((i-1,j));
        }
        if i < m-1 {
            v.push((i+1,j));
        }
        if j > 0 {
            v.push((i,j-1));
        }
        if j < n-1 {
            v.push((i,j+1));
        }
        v
    };

    for d in 0 .. {

        /* make sure this is consuming */
        let mut todo = VecDeque::new();
        mem::swap(&mut todo, &mut visit_queue);
        // println!("{},{}", todo.len(), visit_queue.len());

        if todo.is_empty() {
            break;
        }

        while let Some(current) = todo.pop_back() {
            dist[current] = d;
            reached += 1;
            for next in neighbours(current) {
                if !scheduled[next]
                 && topo[current] + 1 >= topo[next] {
                    visit_queue.push_back(next);
                    scheduled[next] = true;
                }
            }
        }

        assert!(todo.is_empty());

        if reached == m*n {
            break;
        }
    }

    println!("Part one: fewest steps is {}", dist[end]);


    let mut low_nodes = vec![];

    /* Initiate Dijkstra (in reverse) from end node */
    let mut dist = Matrix::new(m, n);
    dist.fill(usize::max_value());
    dist[end] = 0;

    let mut scheduled = Matrix::new(m, n);
    scheduled.fill(true);

    let mut queue = PriorityQueue::new();
    for j in 0..n {
        for i in 0..m {
            let idx = (i,j);
            if topo[idx] == 1 {
                low_nodes.push(idx);
            }
            queue.push(idx,
                       usize::max_value() - dist[idx]);
        }
    }

    while let Some((current, p)) = queue.pop() {

        /* should be unreachable */
        if dist[current] == usize::max_value() {
            break;
        }

        scheduled[current] = false;
        for prev in neighbours(current) {
            /* check that prev can reach current */
            if  scheduled[prev]
             && topo[prev] + 1 >= topo[current] {
                if dist[current] + 1 < dist[prev] {
                    dist[prev] = dist[current] + 1;
                    queue.change_priority(&prev, usize::max_value() - dist[prev]);
                }
            }
        }
    }

    let mut min_start_path_length = usize::max_value();
    for node in low_nodes {
        if dist[node] < min_start_path_length {
            min_start_path_length = dist[node];
        }
    }

    println!("Part two: minimal path with least elevation is {}", min_start_path_length);

    Ok(())
}