use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let mut cnt = 0;

    for mut i in 1..=n {
        let mut sum = 0;
        let num = i;
        loop {
            if i == 0 {break;}
            sum += i % 10;
            i /= 10;
        }
        if a <= sum && sum <= b {cnt += num;}
    }
    println!("{}", cnt);
}
