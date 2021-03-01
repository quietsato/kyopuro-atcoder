use std::collections::BTreeSet;

fn main() {
    const MAX: u64 = 100_000;

    let x: u64 = {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.trim_end().parse().unwrap()
    };

    let ans: usize = {
        let mut availables = BTreeSet::<u64>::new();

        for a in 2u64..MAX {
            let mut t = a;
            loop {
                t *= a;
                if t > x {
                    break;
                } else {
                    availables.insert(t);
                }
            }
        }

        x as usize - availables.iter().count()
    };

    println!("{}", ans);
}

