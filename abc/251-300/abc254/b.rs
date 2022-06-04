use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut map = BTreeMap::new();

    for i in 0..=n - 1 {
        for j in 0..=i {
            let a = if j == 0 || j == i {
                1
            } else {
                map.get(&(i - 1, j - 1)).unwrap() + map.get(&(i - 1, j)).unwrap()
            };
            map.insert((i, j), a);
            print!("{}", a);
            if j < i {
                println!(" ");
            } else {
                println!();
            }
        }
    }
}
