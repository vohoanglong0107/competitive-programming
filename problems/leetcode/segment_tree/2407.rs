// https://leetcode.com/problems/longest-increasing-subsequence-ii/description/
#[derive(Debug)]
struct SegmentTree {
    t: Vec<i32>,
}

impl SegmentTree {
    fn update(&mut self, v: usize, l: usize, r: usize, pos: usize, new_value: i32) {
        if l == r {
            self.t[v] = new_value
        } else {
            let m = (l + r) / 2;
            if pos <= m {
                self.update(v << 1, l, m, pos, new_value);
            } else {
                self.update(v << 1 | 1, m + 1, r, pos, new_value);
            }
            self.t[v] = std::cmp::max(self.t[v << 1], self.t[v << 1 | 1])
        }
    }
    fn query(&self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> Option<i32> {
        // println!("{v} {l} {r} {ql} {qr} {:?}", self.t[v]);
        if ql > r || qr < l {
            return None;
        };

        if l == ql && r == qr {
            return Some(self.t[v]);
        };

        let m = (l + r) / 2;

        let left = self.query(v << 1, l, m, ql, std::cmp::min(qr, m));
        let right = self.query(v << 1 | 1, m + 1, r, std::cmp::max(m + 1, ql), qr);
        std::cmp::max(left, right)
    }
}

pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut tree = SegmentTree {
            t: vec![0; 1e6 as usize],
        };
        // tree.build(&nums, 1, 0, n - 1);
        for i in 0..n {
            // println!("{0:?}", nums[i] - k);
            let res = tree
                .query(
                    1,
                    0,
                    1e5 as usize,
                    std::cmp::max(nums[i] - k, 0).try_into().unwrap(),
                    (nums[i] - 1).try_into().unwrap(),
                )
                .unwrap();
            // println!("res: {res}");
            tree.update(1, 0, 1e5 as usize, nums[i] as usize, res + 1);
        }

        tree.query(1, 0, 1e5 as usize, 0, 1e5 as usize).unwrap()
    }
}
