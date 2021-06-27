use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        (a, b, k): (usize, usize, usize)
    }

    let mut ans = String::new();
    rec(a, b, k, &mut ans);

    println!("{}", ans);

    Ok(())
}

fn rec(a: usize, b: usize, k: usize, ans: &mut String) {
    if a == 0 {
        ans.push_str(&"b".repeat(b));
        return;
    }
    if b == 0 {
        ans.push_str(&"a".repeat(a));
        return;
    }

    let key = {
        let mut key = 1u128;
        for i in 1..=a as u128 {
            key *= (a as u128 + b as u128) - i + 1;
            key /= i;
        }
        (key * a as u128 / (a + b) as u128) as usize
    };

    if k > key {
        ans.push('b');
        rec(a, b - 1, k - key, ans);
        return;
    } else {
        ans.push('a');
        rec(a - 1, b, k, ans);
    }
}

