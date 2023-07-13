use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        y: i32,
    }

    for i in 0..=n {
        for j in 0..=n-i {
            let k = n - i - j;
            if 10000*k + 5000*j + 1000*i == y {
                println!("{} {} {}", k, j, i);
                return;
            }
            
        }
    }
    println!("-1 -1 -1");
}
