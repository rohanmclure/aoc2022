#![feature(iter_collect_into)]
use std::mem;
use std::collections::VecDeque;

use num::integer;

struct Monkey {
    _rank: usize,
    start_items: Vec<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    modulo: usize,
    targets: (usize, usize)
}

impl Monkey {
    fn new(rank: usize,
           start_items: Vec<usize>,
           op: Box<dyn Fn(usize) -> usize>,
           modulo: usize,
           targets: (usize, usize)) -> Self {

        Monkey {
            _rank: rank,
            start_items: start_items,
            op: op,
            modulo: modulo,
            targets: targets
        }
    }

    fn op(&self, old: usize) -> usize {
        (*self.op)(old)
    }

    fn reduce(&self, old: usize) -> usize {
        old / 3
    }

    fn pass(&self, v: usize) -> usize {
        if v % self.modulo == 0 {
            self.targets.0
        } else {
            self.targets.1
        }
    }
}

fn main() {
    let monkeys = vec![Monkey::new(0,
                                   vec![57,58],
                                   Box::new(|x| x * 19),
                                   7,
                                   (2,3)),
                       Monkey::new(1,
                                   vec![66,52,59,79,94,73],
                                   Box::new(|x| x + 1),
                                   19,
                                   (4,6)),
                       Monkey::new(2,
                                   vec![80],
                                   Box::new(|x| x + 6),
                                   5,
                                   (7,5)),
                       Monkey::new(3,
                                   vec![82,81,68,66,71,83,75,97],
                                   Box::new(|x| x + 5),
                                   11,
                                   (5,2)),
                       Monkey::new(4,
                                   vec![55,52,67,70,69,94,90],
                                   Box::new(|x| x * x),
                                   17,
                                   (0,3)),
                       Monkey::new(5,
                                   vec![69,85,89,91],
                                   Box::new(|x| x + 7),
                                   13,
                                   (1,7)),
                       Monkey::new(6,
                                   vec![75,53,73,52,75],
                                   Box::new(|x| x * 7),
                                   2,
                                   (0,4)),
                       Monkey::new(7,
                                   vec![94,60,79],
                                   Box::new(|x| x + 2),
                                   3,
                                   (1,6)),
                      ];

    let num_monkeys = monkeys.len();

    let mut monkey_queues: Vec<VecDeque<usize>> = monkeys.iter().map(|m| {
            let mut v: VecDeque<usize> = VecDeque::new();
            m.start_items.iter().collect_into(&mut v);
            v
        }).collect();

    let mut monkey_inspections: Vec<usize> = vec![];
    monkey_inspections.resize(num_monkeys, 0);

    for _round in 1..=20 {
        for i in 0..num_monkeys {
            let mut items = VecDeque::new();
            mem::swap(&mut items, &mut monkey_queues[i]);

            let insp = &mut monkey_inspections[i];
            let monkey = &monkeys[i];

            for mut worry in items {
                *insp += 1;

                worry = monkey.op(worry);
                worry = monkey.reduce(worry);

                let recv = monkey.pass(worry);
                monkey_queues[recv].push_front(worry);
            }
        }
    }

    monkey_inspections.sort();
    let last_two = &monkey_inspections[num_monkeys-2..];
    println!("Part one: monkey business = {}",
             last_two[0] * last_two[1]);

    /* part two */
    let mut monkey_queues: Vec<VecDeque<usize>> = monkeys.iter().map(|m| {
            let mut v: VecDeque<usize> = VecDeque::new();
            m.start_items.iter().collect_into(&mut v);
            v
        }).collect();

    let lcm = monkeys.iter().map(|m| m.modulo).reduce(integer::lcm).unwrap();

    let mut monkey_inspections: Vec<usize> = vec![];
    monkey_inspections.resize(num_monkeys, 0);

    for _round in 1..=10_000 {
        for i in 0..num_monkeys {
            let mut items = VecDeque::new();
            mem::swap(&mut items, &mut monkey_queues[i]);

            let insp = &mut monkey_inspections[i];
            let monkey = &monkeys[i];

            for mut worry in items {
                *insp += 1;

                worry = worry % lcm;
                worry = monkey.op(worry);

                let recv = monkey.pass(worry);
                monkey_queues[recv].push_front(worry);
            }
        }
    }

    monkey_inspections.sort();
    let last_two = &monkey_inspections[num_monkeys-2..];
    println!("Part two: monkey business = {}",
             last_two[0] * last_two[1]);
}