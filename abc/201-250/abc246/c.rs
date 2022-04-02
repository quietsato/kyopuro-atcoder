use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: i64,
        x: i64,
        mut a: [i64; n]
    }

    a.sort_unstable();
    a.reverse();

    for a in a.iter_mut() {
        let use_num = (*a / x).min(k);
        *a -= x * use_num;
        k -= use_num;
        if k == 0 {
            break;
        }
    }

    a.sort_unstable();
    a.reverse();

    if k > 0 {
        for a in a.iter_mut().take(k as usize) {
            *a = 0;
        }
    } 

    println!("{}", a.iter().sum::<i64>());
}
