#![feature(split_inclusive)]
use itertools::Itertools;
use std::collections::HashSet;

fn bruteforce(v: Vec<u64>, mut parent: HashSet<Vec<u64>>) -> HashSet<Vec<u64>> {
    let mut vecs = HashSet::new();


    for idx in 1..(v.len() - 1) {
        let diff = v[idx+1] - v[idx-1];
        if diff < 4 {
            let mut newv = v.clone();
            newv.remove(idx);
            vecs.insert(newv);
        }
    }

    //println!("{:#?}", vecs);

    for v in vecs.iter() {
        parent.insert(v.clone());

        for v in bruteforce(v.clone(), vecs.clone()) {
            parent.insert(v);
        }
    }

    parent.insert(v);

    //println!("{:#?}", parent);

    parent
}

fn main() {
    let input: Vec<u64> = std::fs::read_to_string("input_day10.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse::<u64>().unwrap()).collect();

    let extra: Vec<u64> = vec![0, input.iter().max().unwrap() + 3];

    let input_i = input.into_iter().chain(extra.into_iter())
        .sorted();
    let diffs = input_i.clone()
        .tuple_windows()
        .map(|(a, b)| b - a);
    let groups0 = input_i.clone()
        .tuple_windows()
        .collect::<Vec<_>>();
    let groups = groups0
        .split_inclusive(|(a, b)| (b - a) == 3)
        .map(|i|i.iter().map(|x| x.0).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    //println!("{:#?}", groups);
    println!("{}", diffs.clone().filter(|x|*x == 1).count() * diffs.filter(|x|*x == 3).count());
    //println!("{:#?}", groups.iter().map(|group| bruteforce(group.clone(), HashSet::new())).collect::<Vec<_>>());
    println!("{:#?}", groups.iter().map(|group| bruteforce(group.clone(), HashSet::new()).len()).fold(1, |a, b| a * b));

    //println!("{}", 1 + bruteforce(input_i.clone().collect(), HashSet::new()).len());
}
