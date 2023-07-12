use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    if n % 2 == 1 {
        println!("Black");
    } else {
        println!("White");
    }
}
