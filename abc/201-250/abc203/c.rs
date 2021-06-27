use itertools::Itertools;
use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        (n, k): (usize, u64),
        mut ab: [(u64, u64); n]
    }

    ab.sort_by(|a, b| a.0.cmp(&b.0));

    let mut village = 0;
    let mut money = k;
    for (a, b) in ab {
        if (a - village) <= money {
            money -= a - village;
            money += b;
            village = a;
        } else {
            break;
        }
    }

    println!("{}", village + money);

    Ok(())
}
