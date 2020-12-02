use itertools::iproduct;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let entries: Vec<u32> = input.split('\n').map(|s| s.parse().unwrap()).collect();

    let (i, j) = iproduct!(&entries, &entries)
        .filter(|(&i, &j)| i + j == 2020)
        .next()
        .expect("No solutions");
    println!("{}", i * j);

    let (i, j, k) = iproduct!(&entries, &entries, &entries)
        .filter(|(&i, &j, &k)| i + j + k == 2020)
        .next()
        .expect("No solutions");
    println!("{}", i * j * k);
}
