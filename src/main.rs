use std::io;

use competitive_programming::*;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let nums = x
        .trim()
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect::<Vec<i32>>();
    x.clear();
    io::stdin().read_line(&mut x).unwrap();
    let k: i32 = x.trim().parse().unwrap();
    x.clear();

    println!("{:?}", Solution::length_of_lis(nums, k))
}
