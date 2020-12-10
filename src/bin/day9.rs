use std::collections::HashSet;
use itertools::iproduct;

fn is_valid(v: &Vec<u64>, idx: usize) -> bool {
    if idx < 25 {
        // preamble
        true
    } else {
        let possible_values = iproduct!(((idx - 25)..idx), ((idx - 25)..idx))
            .filter_map(|(a, b)| if a == b { None } else { Some(v[a] + v[b]) })
            .collect::<HashSet<_>>();
        //println!("Is {} in {:#?}?", v[idx], possible_values);
        possible_values.contains(&v[idx])
    }
}

fn contiguous_starting_at_x(v: &Vec<u64>, idx: usize, target: u64) -> Option<Vec<u64>>{
    for y in 0..(v.len() - idx) {
        let this_segment = v[idx..].iter().take(y);
        let s: u64 = this_segment.clone().sum();
        if s == target {
            return Some(this_segment.map(|x| *x).collect());
        } else if s > target {
            return None;
        }
    }
    return None;
}

fn main() {
    let input = std::fs::read_to_string("input_day9.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    let first_invalid = input
        .iter()
        .enumerate()
        .filter(|(idx, _val)| !is_valid(&input, *idx))
        .next()
        .unwrap()
        .1;
    println!("{:#?}", first_invalid);
    let foo = input.iter().enumerate().filter_map(|(idx, _val)| contiguous_starting_at_x(&input, idx, *first_invalid)).next().unwrap();
    let max = foo.iter().fold(0, |a, b| std::cmp::max(a, *b));
    let min = foo.iter().fold(max, |a, b| std::cmp::min(a, *b));
    println!("{}", max + min);
}
