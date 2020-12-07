use std::collections::{HashMap, HashSet};

fn find_outer_bags<'a>(
    inner_colours: HashSet<&'a str>,
    hm: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    let mut new_inner_colours = inner_colours.clone();
    for c in inner_colours.clone() {
        for (k, v) in hm {
            if v.contains(c) {
                new_inner_colours.insert(*k);
            }
        }
    }

    //println!("{:#?} {:#?}", inner_colours, new_inner_colours);
    if inner_colours == new_inner_colours {
        return inner_colours;
    } else {
        return find_outer_bags(new_inner_colours, hm);
    }
}

fn count_inner_bags<'a>(
    outer_colour: &str,
    hm: &HashMap<&'a str, Vec<&'a str>>,
) -> usize {
    let inner_colours = hm.get(outer_colour).unwrap();
    inner_colours.len() + inner_colours.iter().map(|c| count_inner_bags(c, hm)).sum::<usize>()
}

fn part1() {
    let input = std::fs::read_to_string("input_day7.txt").unwrap();
    let re = regex::Regex::new(r"(\w+ \w+) bags contain (\d.+)?").unwrap();
    let inner_bag_re = regex::Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    let hm = input
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
                    //(c.get(1).unwrap().as_str().parse::<u32>().unwrap(), c.get(2).unwrap().as_str())
                    c.get(2).unwrap().as_str()
                })
                .collect::<HashSet<_>>()}).unwrap_or_default();
            (outer_bag, inner_bags)
        })
        .collect::<HashMap<_, _>>();
    let initial_outer_bags = hm.iter().filter_map(|(k, v)| if v.contains("shiny gold") {Some(*k)} else {None}).collect();
    let outer_bags = find_outer_bags(initial_outer_bags, &hm);
    println!(
        "{:#?}", outer_bags
    );
    println!(
        "{}", outer_bags.len()
    );
    
}

fn part2() {
    let input = std::fs::read_to_string("input_day7.txt").unwrap();
    let re = regex::Regex::new(r"(\w+ \w+) bags contain (\d.+)?").unwrap();
    let inner_bag_re = regex::Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    let hm = input
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
                    let num_bags = c.get(1).unwrap().as_str().parse().unwrap();
                    std::iter::repeat(c.get(2).unwrap().as_str()).take(num_bags)
                })
                .flatten()
                .collect::<Vec<_>>()}).unwrap_or_default();
            (outer_bag, inner_bags)
        })
        .collect::<HashMap<_, _>>();
    println!(
        "{}", count_inner_bags("shiny gold", &hm)
    );
    
}

fn main() {
    part1();
    part2();
}