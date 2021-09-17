use proconio::input;

fn main() {
    input! {
        mut n: u64
    }

    if n == 1 {
        println!("0");
        return;
    }

    println!(
        "{}",
        (2u64..)
            .take_while(move |i| i * i <= n)
            .filter_map(|i| {
                let mut count = 0;
                while n >= i && n % i == 0 {
                    n /= i;
                    count += 1;
                }
                if count == 0 {
                    None
                } else {
                    Some(count)
                }
            })
            .map(|count| {
                let mut count = count;
                let mut ret = 0;
                for j in 1.. {
                    if j > count {
                        break;
                    }
                    count -= j;
                    ret = j;
                }
                ret
            })
            .sum::<u64>()
            + if n > 1 { 1 } else { 0 }
    );
}
