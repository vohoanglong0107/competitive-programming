// https://leetcode.com/problems/booking-concert-tickets-in-groups/description/
#[derive(Debug)]
struct MaxSegmentTree {
    t: Vec<i32>,
}

impl MaxSegmentTree {
    fn build(&mut self, a: &Vec<i32>, v: usize, l: usize, r: usize) {
        if l == r {
            self.t[v] = a[l];
        } else {
            let m = (l + r) / 2;
            self.build(a, v << 1, l, m);
            self.build(a, v << 1 | 1, m + 1, r);
            self.t[v] = std::cmp::max(self.t[v << 1], self.t[v << 1 | 1])
        }
    }
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

#[derive(Debug)]
struct SumSegmentTree {
    t: Vec<i64>,
}
impl SumSegmentTree {
    fn build(&mut self, a: &Vec<i32>, v: usize, l: usize, r: usize) {
        if l == r {
            self.t[v] = a[l] as i64;
        } else {
            let m = (l + r) / 2;
            self.build(a, v << 1, l, m);
            self.build(a, v << 1 | 1, m + 1, r);
            self.t[v] = self.t[v << 1] + self.t[v << 1 | 1]
        }
    }
    fn update(&mut self, v: usize, l: usize, r: usize, pos: usize, new_value: i32) {
        if l == r {
            self.t[v] = new_value as i64
        } else {
            let m = (l + r) / 2;
            if pos <= m {
                self.update(v << 1, l, m, pos, new_value);
            } else {
                self.update(v << 1 | 1, m + 1, r, pos, new_value);
            }
            self.t[v] = self.t[v << 1] + self.t[v << 1 | 1]
        }
    }
    fn query(&self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> Option<i64> {
        if ql > r || qr < l {
            return None;
        };

        if l == ql && r == qr {
            return Some(self.t[v]);
        };

        let m = (l + r) / 2;

        let left = self
            .query(v << 1, l, m, ql, std::cmp::min(qr, m))
            .unwrap_or(0);
        let right = self
            .query(v << 1 | 1, m + 1, r, std::cmp::max(ql, m + 1), qr)
            .unwrap_or(0);
        Some(left + right)
    }
}

#[derive(Debug)]
pub struct BookMyShow {
    n: usize,
    m: i32,
    pub a: Vec<i32>,
    max_tree: MaxSegmentTree,
    sum_tree: SumSegmentTree,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BookMyShow {
    pub fn new(n: i32, m: i32) -> Self {
        let n = n.try_into().unwrap();
        let a = vec![m; n];
        let mut max_tree = MaxSegmentTree { t: vec![0; n << 2] };
        max_tree.build(&a, 1, 0, n - 1);

        let mut sum_tree = SumSegmentTree { t: vec![0; n << 2] };
        sum_tree.build(&a, 1, 0, n - 1);

        let show = BookMyShow {
            n,
            m,
            a,
            max_tree,
            sum_tree,
        };
        show
    }

    pub fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let max_row: usize = max_row.try_into().unwrap();
        let mut l = 0;
        let mut r = max_row + 1;
        let mut pos: Option<usize> = None;
        while l < r {
            let m = (l + r) / 2;
            let vm = self.max_tree.query(1, 0, self.n - 1, 0, m).unwrap();
            if vm < k {
                l = m + 1;
            } else {
                r = m;
                pos = Some(m);
            }
        }
        match pos {
            Some(pos) => {
                let row = pos.try_into().unwrap();
                let col = self.m - self.a[pos];

                self.max_tree.update(1, 0, self.n - 1, pos, self.a[pos] - k);
                self.sum_tree.update(1, 0, self.n - 1, pos, self.a[pos] - k);
                self.a[pos] -= k;
                vec![row, col]
            }
            None => vec![],
        }
    }

    pub fn scatter(&mut self, mut k: i32, max_row: i32) -> bool {
        let max_row: usize = max_row.try_into().unwrap();
        let mut l = 0;
        let mut r = max_row + 1;
        let mut pos: Option<usize> = None;
        let mut s: Option<i64> = None;
        while l < r {
            let m = (l + r) / 2;
            let vm = self.sum_tree.query(1, 0, self.n - 1, 0, m).unwrap();
            if vm < k as i64 {
                l = m + 1;
            } else {
                r = m;
                pos = Some(m);
                s = Some(vm);
            }
        }
        let mut l = 0;
        let mut r = max_row + 1;
        let mut zero_pos = 0;
        while l < r {
            let m = (l + r) / 2;
            let vm = self.sum_tree.query(1, 0, self.n - 1, 0, m).unwrap();
            if vm == 0 {
                l = m + 1;
                zero_pos = m;
            } else {
                r = m;
            }
        }
        match pos {
            Some(pos) => {
                let mut s = s.unwrap();
                for i in zero_pos..pos {
                    s -= self.a[i] as i64;
                    k -= self.a[i];
                    self.sum_tree.update(1, 0, self.n - 1, i, 0);
                    self.max_tree.update(1, 0, self.n - 1, i, 0);
                    self.a[i] = 0;
                }
                let rem = (s - (k as i64)) as i32;
                self.sum_tree.update(1, 0, self.n - 1, pos, rem);
                self.max_tree.update(1, 0, self.n - 1, pos, rem);
                self.a[pos] = rem;
                true
            }
            None => false,
        }
    }
}
