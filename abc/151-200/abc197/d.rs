#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

fn main() {
    let n: u32 = parse_line().unwrap();
    let (x0, y0): (f64, f64) = parse_line().unwrap();
    let (x, y): (f64, f64) = parse_line().unwrap();

    let ans = {
        let (cx, cy) = ((x + x0) / 2.0, (y + y0) / 2.0);

        let theta = std::f64::consts::PI / f64::from(n) * 2.0;

        (
            f64::cos(theta) * (x0 - cx) - f64::sin(theta) * (y0 - cy) + cx,
            f64::sin(theta) * (x0 - cx) + f64::cos(theta) * (y0 - cy) + cy,
        )
    };

    println!("{} {}", ans.0, ans.1);
}
