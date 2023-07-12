use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        mut a: [i32;n],
    }
    
    a.sort();

    let mut sum_a = 0;
    let mut sum_b = 0;


    for i in 0..a.len() {
        if i % 2 == 0 {
            sum_a += a[a.len()-i-1];
        } else {
            sum_b += a[a.len()-i-1];
        }
    }

    println!("{}", sum_a - sum_b);
}