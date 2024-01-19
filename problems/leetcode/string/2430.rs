use std::cmp::*;

pub struct Solution {}

const MOD: u64 = 1e9 as u64 + 7;
const POW: u64 = 256;

fn delete_string(s: String) -> i32 {
    let n = s.len();
    let mut dp = vec![1; n];
    let mut hashes = vec![vec![0; n]; n];
    let chars = s.chars().collect::<Vec<char>>();
    for i in 0..n {
        hashes[i][i] = &chars[i].into() % MOD;
        for j in i + 1..n {
            hashes[i][j] = (hashes[i][j - 1] * POW % MOD + &chars[j].into() % MOD) % MOD;
        }
    }

    for i in (0..n).rev() {
        for j in i + 1..n {
            let e = j - i;
            if 2 * e > n - i {
                continue;
            }
            if hashes[i][j - 1] == hashes[j][j + e - 1] {
                dp[i] = max(dp[i], dp[i + e] + 1)
            }
        }
    }

    dp[0]
}

impl Solution {
    pub fn delete_string(s: String) -> i32 {
        delete_string(s)
    }
}
