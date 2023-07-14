use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut s: String,
    }

    let search_words = ["dream", "dreamer", "erase", "eraser"];

    let mut res = "NO";

    loop {
        if s.is_empty() {
            res = "YES";
            break;
        }

        let s_len = s.len();

        for w in search_words.iter() {
            if s.ends_with(w) {
                s = s[..s.len()-w.len()].to_string();
            }
        }

        if s_len == s.len() {break;}
    }
    println!("{}", res);
}
