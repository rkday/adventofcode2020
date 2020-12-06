use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input_day6.txt").unwrap();

    let groups = input.lines().peekable().batching(|i| {
        i.peek().map(|_| ()).map(|_| {
            i.skip_while(|s| s.is_empty())
                .take_while(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
    });

    let anyone_answered_yes: usize = groups
        .clone()
        .map(|group| {
            ('a'..='z')
                .filter(|chr| group.iter().any(|s| s.contains(*chr)))
                .count()
        })
        .sum();

    let everyone_answered_yes: usize = groups
        .map(|group| {
            ('a'..='z')
                .filter(|chr| group.iter().all(|s| s.contains(*chr)))
                .count()
        })
        .sum();

    println!(
        "Part1 {}, part2 {}",
        anyone_answered_yes, everyone_answered_yes
    );
}
