use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars
    }

    for i in 0.. {
        if s[i] == '1' {
            if i & 1 == 0 {
                println!("Takahashi");
            } else {
                println!("Aoki");
            }
            break;
        }
    }
}
