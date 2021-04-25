use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

use std::mem::swap;

fn main() {
    // let source = AutoSource::from("2
    // FLIP
    // 2
    // 2 0 0
    // 1 1 4");
    input!{
        // from source,
        n: usize,
        s: Chars,
        q: usize,
        tab: [(i32, usize, usize); q]
    };

    // let mut left = &mut s[0..n];
    // let mut right = &mut s[n..2*n];

    let mut left: Vec<char> =vec![];
    let mut right: Vec<char> =vec![];

    for i in 0..n {
        left.push(s[i])
    }
    for i in n..2*n {
        right.push(s[i])
    }

    for (t, a, b) in tab {
        if t == 2{
            // let x = left;
            // left = right;
            // right = x;
            swap(&mut left, &mut right)
        }
        else if t == 1{
            if a <= n && b<= n {
                left.swap(a-1, b-1)
            }
            else if a <= n && b > n {
                let x = right[b - 1 - n];
                right[b - 1 - n] = left[a-1];
                left[a-1] = x;
            }
            else if a > n && b <= n {
                let x = left[b-1];
                left[b-1] = right[a-1-n];
                right[a-1-n] = x;
            }
            else {
                right.swap(b-1-n, a-1-n)
            }

        }
    }

    println!("{}{}", left.iter().collect::<String>(), right.iter().collect::<String>())

}
