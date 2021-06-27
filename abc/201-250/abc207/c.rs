use proconio::input;

fn main() {
    input! {
        n: usize,
        tlr: [(u64, u64, u64); n]
    }

    let mut ans = 0usize;
    for i in 0..n {
        for j in i + 1..n {
            if has_dup(tlr[i], tlr[j]) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

fn has_dup(tlr1: (u64, u64, u64), tlr2: (u64, u64, u64)) -> bool {
    let mut tlr1 = tlr1;
    let mut tlr2 = tlr2;

    tlr1.1 *= 2;
    tlr1.2 *= 2;
    tlr2.1 *= 2;
    tlr2.2 *= 2;

    if tlr1.0 == 2 || tlr1.0 == 4 {
        tlr1.2 -= 1;
    }
    if tlr1.0 == 3 || tlr1.0 == 4 {
        tlr1.1 += 1;
    }
    if tlr2.0 == 2 || tlr2.0 == 4 {
        tlr2.2 -= 1;
    }
    if tlr2.0 == 3 || tlr2.0 == 4 {
        tlr2.1 += 1;
    }

    (tlr1.1 <= tlr2.1 && tlr1.2 >= tlr2.1) || (tlr2.1 <= tlr1.1 && tlr2.2 >= tlr1.1)
}
