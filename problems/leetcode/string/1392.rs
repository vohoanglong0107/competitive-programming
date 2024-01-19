pub struct Solution {}

const MOD: u64 = 1e9 as u64 + 7;
const POW: u64 = 256;

fn longest_prefix(s: String) -> String {
    let n = s.len();
    let mut res = None;
    let mut forward_hashes = vec![0; n];
    let mut reverse_hashes = vec![0; n];
    let chars = s.chars().collect::<Vec<char>>();
    forward_hashes[0] = &chars[0].into() % MOD;
    for i in 1..n {
        forward_hashes[i] = (forward_hashes[i - 1] * POW % MOD + &chars[i].into() % MOD) % MOD;
    }
    reverse_hashes[n - 1] = &chars[n - 1].into() % MOD;
    let mut poww = POW;
    for i in (0..n - 1).rev() {
        reverse_hashes[i] = (reverse_hashes[i + 1] + &chars[i].into() * poww % MOD) % MOD;
        poww = poww * POW % MOD;
    }

    for i in 0..n - 1 {
        if forward_hashes[i] == reverse_hashes[n - i - 1] {
            res = Some(i + 1);
        }
    }

    match res {
        None => String::from(""),
        Some(l) => String::from(&chars[0..l].into_iter().collect::<String>()),
    }
}

impl Solution {
    pub fn longest_prefix(s: String) -> String {
        longest_prefix(s)
    }
}
