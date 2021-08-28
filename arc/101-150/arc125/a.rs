use proconio::input;

fn main() {
    let ans: Option<usize> = solve();
    println!(
        "{}",
        ans.map_or_else(|| { (-1).to_string() }, |ans| { ans.to_string() })
    );
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        m: usize,
        s: [u8; n],
        t: [u8; m]
    }
    let mut ans = 0;

    let mut switched = false;

    let mut a1 = s[0];
    for c in t {
        if c != a1 {
            if switched {
                ans += 1;
            } else {
                let mut add = n;
                for d in 1..n {
                    let pi = (d) % n;
                    let mi = (n - d) % n;
                    if s[pi] == c || s[mi] == c {
                        add = d;
                        break;
                    }
                }
                if add == n {
                    return None;
                } else {
                    ans += add;
                }
                switched = true;
            }
            a1 ^= 1;
        }
        ans += 1;
    }

    Some(ans)
}
