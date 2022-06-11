use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; k],
        xy: [(f64, f64); n]
    }

    let mut max_d = 0.0f64;
    for i in 0..n {
        let mut min_d = 10.0f64.powi(6);
        for &j in &a {
            let (x0, y0) = xy[i];
            let (x1, y1) = xy[j];
            let d = ((x0 - x1).powi(2) + (y0 - y1).powi(2)).sqrt();
            min_d = min_d.min(d);
        }
        max_d = max_d.max(min_d);
    }

    println!("{}", max_d);
}
