use proconio::input;

fn main() {
    input! {
        mut n: i64
    }

    for i in 1.. {
        n -= i;
        if n <= 0 {
            println!("{}", i);
            break;
        }
    }
}
