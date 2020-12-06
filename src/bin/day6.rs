use itertools::Itertools;

fn part1() {
    let input = std::fs::read_to_string("input_day6.txt").unwrap();

    let groups: usize = input
        .lines()
        .batching(|i| {
            i.skip_while(|s| s.is_empty())
                .take_while(|s| !s.is_empty())
                .fold(None, |acc, s| {
                    Some(format!("{}{}", acc.unwrap_or_else(String::new), s))
                })
        })
        .map(|group: String| group.chars().unique().count())
        .sum();

    println!("{:#?}", groups);
}

fn part2() {
    let input = std::fs::read_to_string("input_day6.txt").unwrap();

    let groups = input.lines().peekable().batching(|i| {
        if i.peek().is_some() {
            Some(
                i.skip_while(|s| s.is_empty())
                    .take_while(|s| !s.is_empty())
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
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

fn main() {
    part1();
    part2();
}
