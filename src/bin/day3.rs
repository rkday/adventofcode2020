fn part1() {
    let input = std::fs::read_to_string("input_day3.txt").unwrap();
    let mut trees = 0;
    for (linenum, line) in input.lines().enumerate() {
        let pos = (0 + (3 * linenum)) % line.len();
        if line.chars().collect::<Vec<_>>()[pos] == '#' {
            trees += 1;
        }
    }
    println!("{}", trees);
}

fn part2(right: usize, down: usize) -> usize {
    std::fs::read_to_string("input_day3.txt")
        .unwrap()
        .lines()
        .step_by(down)
        .enumerate()
        // Skip the lines we're moving down past
        .filter(|(linenum, line)| {
            // Move right as many squares as we move down, but then wrap that back in to the actual width of our (infinitely-repeating) input
            let pos = (right * linenum) % line.len();
            // Did we hit a tree?
            line.chars().nth(pos) == Some('#')
        })
        .count()
}

fn main() {
    part1();

    println!("{}", part2(1, 1));
    println!("{}", part2(3, 1));
    println!("{}", part2(5, 1));
    println!("{}", part2(7, 1));
    println!("{}", part2(1, 2));
    println!(
        "{}",
        part2(1, 1) * part2(3, 1) * part2(5, 1) * part2(7, 1) * part2(1, 2)
    );
}
