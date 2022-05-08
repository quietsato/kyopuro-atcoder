use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    for y in 0..n {
        for _ in 0..a {
            for x in 0..n {
                for _ in 0..b {
                    if (x + y) % 2 == 0 {
                        print!(".");
                    } else {
                        print!("#");
                    }
                }
            }
            println!();
        }
        println!();
    }
}
