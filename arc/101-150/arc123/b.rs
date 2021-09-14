use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        mut b: [u64; n],
        mut c: [u64; n],
    }

    let a = {
        a.sort();
        a
    };
    let b = {
        b.sort();
        b
    };
    let c = {
        c.sort();
        c
    };

    let (mut ai, mut bi, mut ci) = (0, 0, 0);

    let mut ans = 0u64;
    loop {
        while bi < n && a[ai] >= b[bi] {
            bi += 1;
        }
        if bi == n {
            break;
        }
        while ci < n && b[bi] >= c[ci] {
            ci += 1;
        }
        if ci == n {
            break;
        } 
        
        ai += 1;
        bi += 1;
        ci += 1;
        ans += 1;
    }

    println!("{}", ans);
}
