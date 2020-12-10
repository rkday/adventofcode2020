use itertools::Itertools;
fn main() {
    let input: Vec<u64> = std::fs::read_to_string("input_day10.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse::<u64>().unwrap()).collect();

    let extra: Vec<u64> = vec![0, input.iter().max().unwrap() + 3];

    let diffs = input.into_iter().chain(extra.into_iter())
        .sorted()
        .tuple_windows()
        .map(|(a, b)| b - a);
       // .collect::<Vec<u64>>();
    println!("{}", diffs.clone().filter(|x|*x == 1).count() * diffs.filter(|x|*x == 3).count());
}
