use proconio::input;

fn main() {
    input! {
        k: u64
    }

    let mut v = vec![];
    for i in 0..64 - k.leading_zeros() {
        let i = i as usize;
        if is_nth_bit_one(k, i) {
            v.push(2);
        } else {
            v.push(0);
        }
    }
    v.reverse();

    for b in v {
        print!("{}", b);
    }
    println!();
}

fn is_nth_bit_one(bits: u64, n: usize) -> bool {
    (bits >> n) & 1 == 1
}
