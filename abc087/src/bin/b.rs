use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    }

    let mut cnt = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if i*500 + j*100 + 50*k == x {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt);
}
