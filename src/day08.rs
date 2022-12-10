
use std::env;
use std::io::{self, BufReader};
use std::fs;
use std::ptr::NonNull;

use lazy_static::lazy_static;

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

    let height_grid = {
        let mut lines = vec![];
        while let Some(line) = parse_non_empty_line(&mut r) {
            lines.push(line);
        }

        let mut g = Matrix::<u8>::new(lines.len(), lines[0].len());
        let (m, n) = g.get_dims();

        for i in 0 .. m {
            let line = &lines[i];
            for j in 0 .. n {
                g[(i,j)] = line.get(j..j+1)
                                .unwrap()
                                .parse()
                                .unwrap();
            }
        }
        g
    };

    let (m, n) = height_grid.get_dims();

    let mut known_visible = Matrix::<bool>::new(m, n);
    known_visible.fill(true);

    let mut scenic_scores = Matrix::<usize>::new(m, n);
    scenic_scores.fill(0);

    {
        let v = &mut known_visible;
        let s = &mut scenic_scores;
        let g = &height_grid;

        /* Outer shell is always visible */
        for j in 1 .. n-1 {
            for i in 1 .. m-1 {
                let h = g[(i, j)];
                let mut scores = vec![];
                let mut visible = false;

                /* left */
                let max = (0 .. j).rev()
                                  .map(|jj| g[(i,jj)])
                                  .max().unwrap();
                let vd = (0 .. j).rev().enumerate()
                                 .map_while(|(k, jj)|
                                      if g[(i,jj)] < h {
                                          if jj == 0 {
                                              Some(k+1)
                                          } else {
                                              Some(k+2)
                                          }
                                      } else {
                                          None
                                      })
                                 .last().unwrap_or(1);
                if h > max {
                    visible = true;
                }
                scores.push(vd);

                /* right */
                let max = (j+1 .. n).map(|jj| g[(i,jj)])
                                    .max().unwrap();
                let vd = (j+1 .. n).enumerate()
                                   .map_while(|(k, jj)|
                                        if g[(i,jj)] < h {
                                            if jj == n-1 {
                                                Some(k+1)
                                            } else {
                                                Some(k+2)
                                            }
                                        } else {
                                            None
                                        })
                                   .last().unwrap_or(1);
                if h > max {
                    visible = true;
                }
                scores.push(vd);

                /* up */
                let max = (0 .. i).rev()
                                  .map(|ii| g[(ii,j)])
                                  .max().unwrap();
                let vd = (0 .. i).rev().enumerate()
                                 .map_while(|(k, ii)|
                                    if g[(ii,j)] < h {
                                        if ii == 0 {
                                            Some(k+1)
                                        } else {
                                            Some(k+2)
                                        }
                                    } else {
                                        None
                                    })
                                 .last().unwrap_or(1);
                if h > max {
                    visible = true;
                }
                scores.push(vd);

                /* down */
                let max = (i+1 .. m).map(|ii| g[(ii,j)])
                                    .max().unwrap();
                let vd = (i+1 .. m).enumerate()
                                   .map_while(|(k, ii)|
                                        if g[(ii,j)] < h {
                                            if ii == m-1 {
                                                Some(k+1)
                                            } else {
                                                Some(k+2)
                                            }
                                        } else {
                                            None
                                        })
                                   .last().unwrap_or(1);
                if h > max {
                    visible = true;
                }
                scores.push(vd);

                println!("({}, {}): {:?}, prod: {}", i, j, scores, scores.iter().product::<usize>());
                v[(i,j)] = visible;
                s[(i,j)] = scores.iter().product();
            }
        }
    }

    /* Count number of visible */
    let mut num_visible = 0;
    for j in 0 .. n {
        for i in 0 .. m {
            if known_visible[(i,j)] {
                num_visible += 1;
            }
        }
    }
    println!("Part one: number of visible trees: {num_visible}");

    /* Find maximum scenic score */
    let mut max_scenic = 0;
    for j in 0 .. n {
        for i in 0 .. m {
            let s = scenic_scores[(i,j)];
            if s > max_scenic {
                max_scenic = s;
            }
        }
    }
    println!("Part two: maximum scenic score: {max_scenic}");

    Ok(())
}