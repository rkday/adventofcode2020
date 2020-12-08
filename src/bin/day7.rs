use std::collections::{HashMap, HashSet};

fn find_outer_bags<'a>(
    inner_colours: HashSet<&'a str>,
    hm: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    let new_inner_colours = inner_colours
        .iter()
        .map(|c| hm.iter().filter_map(move |(k, v)| v.get(c).map(|_| *k)))
        .flatten()
        .collect::<HashSet<_>>()
        .union(&inner_colours)
        .map(|x| *x)
        .collect::<HashSet<_>>();

    if inner_colours == new_inner_colours {
        inner_colours
    } else {
        find_outer_bags(new_inner_colours, hm)
    }
}

fn count_inner_bags<'a>(outer_colour: &str, hm: &HashMap<&'a str, Vec<&'a str>>) -> usize {
    let inner_colours = hm.get(outer_colour).unwrap();
    inner_colours.len()
        + inner_colours
            .iter()
            .map(|c| count_inner_bags(c, hm))
            .sum::<usize>()
}

fn main() {
    let input = std::fs::read_to_string("input_day7.txt").unwrap();
    let re = regex::Regex::new(r"(\w+ \w+) bags contain (\d.+)?").unwrap();
    let inner_bag_re = regex::Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    let hm_part2 = input
        .lines()
        .map(|i| {
            let c = re.captures(i).unwrap();
            let outer_bag = c.get(1).unwrap().as_str();
            let inner_bags = c
                .get(2)
                .map(|s| {
                    s.as_str()
                        .split(", ")
                        .map(|b| {
                            let c = inner_bag_re.captures(b).unwrap();
                            let num_bags = c[1].parse().unwrap();
                            std::iter::repeat(c.get(2).unwrap().as_str()).take(num_bags)
                        })
                        .flatten()
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            (outer_bag, inner_bags)
        })
        .collect::<HashMap<_, _>>();

    let hm_part1 = hm_part2
        .iter()
        .map(|(k, v)| (*k, v.iter().map(|x| *x).collect::<HashSet<&str>>()))
        .collect::<HashMap<_, _>>();

    let initial_outer_bags = hm_part1
        .iter()
        .filter_map(|(k, v)| v.get("shiny gold").map(|_| *k))
        .collect();
    let outer_bags = find_outer_bags(initial_outer_bags, &hm_part1);
    println!(
        "Part 1 {}, Part 2 {}",
        outer_bags.len(),
        count_inner_bags("shiny gold", &hm_part2)
    );
}
