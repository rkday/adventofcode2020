use im::HashSet;
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{char, digit1},
    sequence::tuple,
};
use std::str::FromStr;
#[derive(Clone, Copy)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Op {
    type Err = ();
    fn from_str(operation: &str) -> Result<Self, Self::Err> {
        let result: nom::IResult<_, _> = tuple((
            alt((tag("nop"), tag("acc"), tag("jmp"))),
            char(' '),
            alt((char('+'), char('-'))),
            digit1,
        ))(operation);

        match result {
            Ok((_, (operation, _, sign, val))) => {
                let i = val.parse::<i32>().unwrap() * if sign == '-' { -1 } else { 1 };
                Ok(match operation {
                    "nop" => Op::Nop(i),
                    "acc" => Op::Acc(i),
                    "jmp" => Op::Jmp(i),
                    _ => unreachable!(),
                })
            }
            _ => Err(()),
        }
    }
}

struct Program {
    operations: Vec<Op>,
}

#[derive(Default)]
struct ExecutionState {
    acc: i32,
    history: HashSet<usize>,
}

impl ExecutionState {
    fn update(&self, increment: i32, hist_item: usize) -> Self {
        Self {
            acc: self.acc + increment,
            history: self.history.update(hist_item),
        }
    }
}

impl Program {
    fn new(operations: Vec<Op>) -> Self {
        Self { operations }
    }

    fn execute(&self) -> Result<i32, i32> {
        self.execute2(0, ExecutionState::default())
    }

    fn execute2(&self, idx: usize, ex: ExecutionState) -> Result<i32, i32> {
        if ex.history.contains(&idx) {
            Err(ex.acc)
        } else if idx == self.operations.len() {
            Ok(ex.acc)
        } else {
            let (increment, advance) = match self.operations[idx] {
                Op::Acc(acc) => (acc, 1),
                Op::Jmp(offset) => (0, offset),
                Op::Nop(_) => (0, 1),
            };

            let new_state = ex.update(increment, idx);

            self.execute2((idx as i32 + advance) as usize, new_state)
        }
    }

    fn attempt_fix(&self, change_idx: usize) -> Self {
        let new_instructions = self
            .operations
            .clone()
            .into_iter()
            .enumerate()
            .map(|(idx, val)| match (idx == change_idx, val) {
                (true, Op::Nop(x)) => Op::Jmp(x),
                (true, Op::Jmp(x)) => Op::Nop(x),
                _ => val,
            })
            .collect();

        Program::new(new_instructions)
    }
}

fn main() {
    let input: Vec<Op> = std::fs::read_to_string("input_day8.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    let p = Program::new(input.clone());
    println!("{}", p.execute().unwrap_or_else(|i| i));

    let indexes_to_change = input.iter().enumerate().filter_map(|(idx, val)| match val {
        Op::Jmp(_) | Op::Nop(_) => Some(idx),
        _ => None,
    });
    let maybe_programs = indexes_to_change.map(|idx| p.attempt_fix(idx));
    let working_program = maybe_programs.filter_map(|p| p.execute().ok()).next();
    println!("{:#?}", working_program.unwrap());
}
