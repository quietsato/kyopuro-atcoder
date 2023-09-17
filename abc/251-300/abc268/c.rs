use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    };

    let mut num_happies = vec![0; n];

    for (i, p) in p.iter().enumerate() {
        num_happies[(n + p - i + 1) % n] += 1;
        num_happies[(n + p - i) % n] += 1;
        num_happies[(n + p - i - 1) % n] += 1;
    }

    let ans = num_happies.iter().max().unwrap();
    println!("{}", ans);
}
