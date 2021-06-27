use proconio::input;

fn main() {
    input! {
        mut ns: [i64; 3]
    }
    ns.sort();
    ns.reverse();

    println!("{}", ns[..2].iter().sum::<i64>());
}
