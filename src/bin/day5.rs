use std::collections::HashSet;

fn chop(seats: Vec<u32>, decider: char) -> Vec<u32> {
    if decider == 'F' || decider == 'L' {
        seats.split_at(seats.len() / 2).0.to_vec()
    } else {
        seats.split_at(seats.len() / 2).1.to_vec()
    }
}
fn main() {
    let row_numbers: Vec<_> = (0..128).collect();
    let col_numbers: Vec<_> = (0..8).collect();

    let input = std::fs::read_to_string("input_day5.txt").unwrap();

    let boarding_passes = input.lines().map(|boarding_pass| {
        let (row, col) = boarding_pass.split_at(7);
        (
            row.chars().fold(row_numbers.clone(), chop)[0],
            col.chars().fold(col_numbers.clone(), chop)[0],
        )
    });

    let all_ids: HashSet<_> = boarding_passes.map(|(r, c)| (r * 8) + c).collect();
    let max_id = all_ids.iter().fold(0, |a, b| std::cmp::max(a, *b));
    let min_id = all_ids.iter().fold(max_id, |a, b| std::cmp::min(a, *b));
    let my_bpid = (min_id..max_id).filter(|id| !all_ids.contains(id)).next();

    println!("max id {}, my id {}", max_id, my_bpid.unwrap());
}
