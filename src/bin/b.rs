use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: usize,
        a: [i32; n],
        b: [i32; n]
    };

    let &min = a.iter().min().unwrap();
    let &max = b.iter().max().unwrap();

    let mut ans = 0;

    for m in min..=max{
        let mut flg = true;
        for i in 0..n{
            if !(a[i] <= m && m <= b[i]){
                flg = false;
                break;
            }
        }
        if flg {
            ans += 1;
        }
    }

    println!("{}", ans);

}
