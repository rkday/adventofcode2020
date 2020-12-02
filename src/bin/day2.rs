use nom::{
    // see the "streaming/complete" paragraph lower for an explanation of these submodules
    character::complete::{anychar, char, digit1},
    sequence::Tuple,
    IResult,
};

fn before_the_colon(input: &str) -> IResult<&str, (&str, char, &str, char, char, char, char)> {
    (
        digit1,
        char('-'),
        digit1,
        char(' '),
        anychar,
        char(':'),
        char(' '),
    )
        .parse(input)
}

fn check(s: &str) -> bool {
    let (remaining, (min_s, _, max_s, _, character, _, _)) = before_the_colon(s).unwrap();
    let (min, max) = (min_s.parse().unwrap(), max_s.parse().unwrap());

    let num_chars = remaining.chars().filter(|&c| c == character).count();
    !(num_chars < min || num_chars > max)
}

fn check_part2(s: &str) -> bool {
    let (remaining, (pos1_s, _, pos2_s, _, character, _, _)) = before_the_colon(s).unwrap();
    let (pos1, pos2): (usize, usize) = (pos1_s.parse().unwrap(), pos2_s.parse().unwrap());

    let vchars: Vec<_> = remaining.chars().collect();
    (vchars[pos1 - 1] == character) ^ (vchars[pos2 - 1] == character)
}

fn main() {
    let input = std::fs::read_to_string("input_day2.txt").unwrap();
    println!(
        "{}",
        input
            .split('\n')
            .filter_map(|line| if line != "" { Some(check(line)) } else { None })
            .filter(|&x| x)
            .count()
    );
    println!(
        "{}",
        input
            .split('\n')
            .filter_map(|line| if line != "" {
                Some(check_part2(line))
            } else {
                None
            })
            .filter(|&x| x)
            .count()
    );
}
