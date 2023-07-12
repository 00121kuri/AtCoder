use proconio::{fastout, input};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut cnt = 0;

    for c in s {
        if c == '1' {cnt += 1;}
    }
    println!("{}", cnt);
}
