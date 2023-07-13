use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        mut d: [i32; n],
    }

    d.sort();
    d.reverse();

    let mut cnt = 0;

    // 重複を数える
    for i in 1..d.len() {
        if d[i-1] == d[i] {cnt += 1;}
    }

    println!("{}", d.len() - cnt);
}
