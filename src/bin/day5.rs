use itertools::iproduct;
use std::collections::HashSet;

#[derive(PartialEq, Hash, Eq, Debug)]
struct BoardingPass {
    pub row: u32,
    pub col: u32,
}

impl BoardingPass {
    fn id(&self) -> u32 {
        self.col + (self.row * 8)
    }
}

fn chop(seats: Vec<u32>, decider: char) -> Vec<u32> {
    let (lower, higher) = seats.split_at(seats.len() / 2);
    if decider == 'F' || decider == 'L' {
        lower
    } else {
        higher
    }
    .to_vec()
}
fn main() {
    let row_numbers: Vec<_> = (0..128).collect();
    let col_numbers: Vec<_> = (0..8).collect();

    let input = std::fs::read_to_string("input_day5.txt").unwrap();

    let boarding_passes = input.lines().map(|boarding_pass| {
        let (row, col) = boarding_pass.split_at(7);
        BoardingPass {
            row: row.chars().fold(row_numbers.clone(), chop)[0],
            col: col.chars().fold(col_numbers.clone(), chop)[0],
        }
    });

    let all_ids: HashSet<_> = boarding_passes.clone().map(|bp| bp.id()).collect();
    let max_id = all_ids.iter().fold(0, |a, b| std::cmp::max(a, *b));

    let known_boarding_passes: HashSet<_> = boarding_passes.collect();

    let all_boarding_passes: HashSet<_> = iproduct!(row_numbers, col_numbers)
        .map(|(row, col)| BoardingPass { row, col })
        .collect();

    let my_boarding_pass_id = all_boarding_passes
        .difference(&known_boarding_passes)
        .filter(|bp| all_ids.contains(&(bp.id() + 1)) && all_ids.contains(&(bp.id() - 1)))
        .next()
        .unwrap()
        .id();
    println!("{}", max_id);
    println!("{}", my_boarding_pass_id);
}
