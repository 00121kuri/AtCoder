use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        a_list: [i32;n],
    }

    let mut cnt = 0;

    loop {
        let mut flag = false;
        for a in &a_list {
            if a % (2_i32.pow(cnt+1)) != 0 {
                flag = true;
            }
        }
        if flag == true {break;}

        cnt += 1;
    }

    println!("{}", cnt);
}
