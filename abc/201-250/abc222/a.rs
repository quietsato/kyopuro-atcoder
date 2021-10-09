use proconio::input;

fn main() {
    input! {
        mut n: String
    }

    while n.len() < 4 {
        n.insert(0, '0');
    }
    println!("{}", n);
}
