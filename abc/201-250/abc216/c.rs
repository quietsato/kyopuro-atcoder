use proconio::input;

fn main() {
    input! {
        n: u64
    }

    let mut ans = vec![];

    for i in 0.. {
        if n & (1 << i) > 0 {
            ans.push('A')
        }
        if n <= (1 << i) {
            break;
        }
        ans.push('B');
    }

    ans.reverse();

    println!("{}", ans.iter().collect::<String>());
}
