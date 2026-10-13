use std::io::Read;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();

    let v: Vec<i64> = s
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let a = v.get(0).copied().unwrap_or(0);
    let b = v.get(1).copied().unwrap_or(0);

    println!("{}", a + b);
}