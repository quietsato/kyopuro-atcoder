use proconio::input;

macro_rules! chmax {
    ($target:expr, $($v: expr),+) => {{
        $target = $target$(.max($v))*;
    }};
}

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![];
    for i in 0..2 * n {
        input! {
            row: [u64; 2 * n - i - 1]
        }
        a.push(row);
    }

    // let mut ans = 0;
    // for perm in (0..2 * n).permutations(2 * n) {
    //     let mut aff = 0;
    //     for i in 0..n {
    //         let x = perm[2 * i].min(perm[2 * i + 1]);
    //         let y = perm[2 * i].max(perm[2 * i + 1]);
    //         aff = aff.bitxor(a[x][y - x - 1]);
    //     }
    //     chmax!(ans, aff);
    // }

    println!("{}", f(0, n, &a, &mut vec![]));
}

fn f(used: u32, n: usize, a: &Vec<Vec<u64>>, pairs: &mut Vec<(usize, usize)>) -> u64 {
    if (0..2 * n).map(|i| (used >> i) & 1 == 1).all(|b| b) {
        let mut ret = 0;
        for (x, y) in pairs {
            ret ^= a[*x][*y - *x - 1];
        }
        return ret;
    }

    let x = (0..2 * n)
        .map(|i| (used >> i) & 1 == 1)
        .take_while(|&b| b)
        .count();
    let used = used | (1 << x);

    let mut ret = 0;
    for y in x + 1..2 * n {
        if (used >> y) & 1 == 1 {
            continue;
        }
        pairs.push((x, y));
        chmax!(ret, f(used | (1 << y), n, a, pairs));
        pairs.pop().unwrap();
    }

    return ret;
}
