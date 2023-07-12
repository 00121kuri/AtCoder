use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x1: i32,
        x2: i32,
        x3: i32,
        x4: i32,
        x5: i32,
    }
    println!("{}",
        if x1 == 0 {
            1
        } else if x2 == 0 {
            2
        } else if x3 == 0 {
            3
        } else if x4 == 0 {
            4
        } else {
            5
        }
    );
}
