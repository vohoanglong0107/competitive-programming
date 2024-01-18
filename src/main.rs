use std::io;

use competitive_programming::*;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let [n, m] = x.trim().split_whitespace().collect::<Vec<&str>>()[..] else {
        panic!()
    };

    let n: i32 = n.parse().unwrap();
    let m: i32 = m.parse().unwrap();

    x.clear();
    io::stdin().read_line(&mut x).unwrap();
    let q: usize = x.trim().parse().unwrap();
    let mut myshow = BookMyShow::new(n, m);
    for _ in 0..q {
        x.clear();
        io::stdin().read_line(&mut x).unwrap();
        let [t, a, b] = x.trim().split_whitespace().collect::<Vec<&str>>()[..] else {
            panic!()
        };
        let t: bool = t.parse().unwrap();
        let a: i32 = a.parse().unwrap();
        let b: i32 = b.parse().unwrap();
        if t == false {
            println!("{:?}", myshow.gather(a, b));
        } else {
            println!("{:?}", myshow.scatter(a, b));
        }
        println!("{:?}", myshow.a)
    }
}
