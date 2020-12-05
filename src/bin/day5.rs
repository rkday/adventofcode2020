use itertools::iproduct;
use itertools::Itertools;
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

    let all_ids: HashSet<_> = boarding_passes.map(|bp| bp.id()).collect();
    let max_id = all_ids.iter().fold(0, |a, b| std::cmp::max(a, *b));

    let my_boarding_pass_id = iproduct!(row_numbers, col_numbers)
        .map(|(row, col)| BoardingPass { row, col }.id())
        .tuple_windows()
        .filter(|(a, b, c)| all_ids.contains(a) && all_ids.contains(c) && !all_ids.contains(b))
        .next()
        .unwrap()
        .1;
    println!("max id {}, my id {}", max_id, my_boarding_pass_id);
}
