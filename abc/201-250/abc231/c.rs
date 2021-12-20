use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u64; n],
        x: [u64; q]
    }

    a.sort();
    let a = a;

    for x in x {
        match a.binary_search(&x) {
            Ok(i) => {
                println!("{}", n - i);
            }
            Err(i) => {
                if i == a.len() {
                    println!("0");
                } else {
                    println!("{}", n - i);
                }
            }
        }
    }
}
