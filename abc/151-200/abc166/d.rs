use proconio::input;

fn main() {
    input! {
        x: i128
    }

    for a in -118i128..=119 {
        for b in -119i128..=118 {
            if (a.pow(5) - b.pow(5)) != x {
                continue;
            }
            println!("{} {}", a, b);
            return;
        }
    }
}
