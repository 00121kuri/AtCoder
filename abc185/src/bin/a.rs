use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [i32; 4],
    }

    let mut min_a = a[0]; 
    for &i in &a {
        if min_a > i {min_a = i}
    }
    println!{"{}", min_a};
}
