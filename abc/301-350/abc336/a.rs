use proconio::input;

fn main() {
    input! {
        n: usize
    };
    println!("L{}ng", vec!["o"; n].join(""));
}
