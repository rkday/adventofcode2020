use std::collections::HashSet;
use std::str::FromStr;
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{char, digit1},
    sequence::tuple,
};
#[derive(Clone)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}


impl FromStr for Op {
    type Err = ();

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(operation: &str) -> Result<Self, Self::Err> {
        let result: nom::IResult<_, _> = tuple(
            (
                alt((tag("nop"), tag("acc"), tag("jmp"))),
                char(' '),
                alt((char('+'), char('-'))),
                digit1
            ))(operation);

        match result {
            Ok((_, ("nop", _, sign, val))) => Ok(Op::Nop(val.parse::<i32>().unwrap() * if sign == '-' { -1 } else {1})),
            Ok((_, ("acc", _, sign, val))) => Ok(Op::Acc(val.parse::<i32>().unwrap() * if sign == '-' { -1 } else {1})),
            Ok((_, ("jmp", _, sign, val))) => Ok(Op::Jmp(val.parse::<i32>().unwrap() * if sign == '-' { -1 } else {1})),
            _ => Err(())

        }
    }
}

struct Program {
    operations: Vec<Op>,
    acc: i32,
    history: HashSet<usize>,
}

impl Program {
    fn new(operations: Vec<Op>) -> Self {
        Program {
            operations,
            acc: 0,
            history: HashSet::new(),
        }
    }
    fn execute(&mut self, idx: usize) -> (bool, i32) {
        if self.history.contains(&idx) {
            (false, self.acc)
        } else if idx == self.operations.len() {
            (true, self.acc)
        } else {
            self.history.insert(idx);

            let (increment, advance) = match self.operations[idx] {
                Op::Acc(acc) => (acc, 1),
                Op::Jmp(offset) => (0, offset),
                Op::Nop(_) => (0, 1),
            };

            self.acc += increment;
            self.execute((idx as i32 + advance) as usize)
        }
    }

    fn attempt_fix(&self, change_idx: usize) -> Self {
        let new_instructions = self.operations.clone().into_iter().enumerate().map(|(idx, val)| {
            if idx == change_idx {
                match val {
                    Op::Nop(x) => Op::Jmp(x),
                    Op::Jmp(x) => Op::Nop(x),
                    _ => panic!()
                }

            } else {
                val
            }
        }).collect();

        Program::new(new_instructions)
    }
}

fn main() {
    let test = vec![
        Op::Nop(0),
        Op::Acc(1),
        Op::Jmp(4),
        Op::Acc(3),
        Op::Jmp(-3),
        Op::Acc(-99),
        Op::Acc(1),
        Op::Jmp(-4),
        Op::Acc(6),
    ];
    let mut p = Program::new(test);
    println!("{}", p.execute(0).1);
    let input: Vec<Op> = std::fs::read_to_string("input_day8.txt").unwrap().lines().map(|l| l.parse().unwrap()).collect();
    let mut p = Program::new(input.clone());
    println!("{}", p.execute(0).1);

    let indexes_to_change = input.iter().enumerate().filter_map(|(idx, val)| match val {
        Op::Jmp(_) | Op::Nop(_) => Some(idx),
        _ => None
    });
    let maybe_programs = indexes_to_change.map(|idx| p.attempt_fix(idx));
    let working_program = maybe_programs.map(|mut p| p.execute(0)).filter(|(terminates, _)| *terminates).next();
    println!("{:#?}", working_program);
}
