fn trees_for_slope(right: usize, down: usize) -> usize {
    std::fs::read_to_string("input_day3.txt")
        .unwrap()
        .lines()
        .step_by(down)
        .enumerate()
        .filter(|(linenum, line)| {
            // Move right with each step, but then wrap that back in to the actual width of our (infinitely-repeating) input
            let pos = (right * linenum) % line.len();
            // Did we hit a tree?
            line.chars().nth(pos) == Some('#')
        })
        .count()
}

fn main() {
    println!("Part 1: {}", trees_for_slope(3, 1));

    let result = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(right, down)| {
            let trees = trees_for_slope(right, down);
            println!("{} right, {} down - {} trees", right, down, trees);
            trees
        })
        .fold(1, |acc, x| acc * x);

    println!("Part 2: {}", result);
}
