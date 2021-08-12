use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64
    }

    let h_deg = -h * (2.0 * PI / 12.0) - m * (2.0 * PI / 60.0 / 12.0);
    let m_deg = -m * (2.0 * PI / 60.0);

    let (hx, hy) = (
        h_deg.cos() * 0.0 + h_deg.sin() * a,
        -h_deg.sin() * 0.0 + h_deg.cos() * a,
    );
    let (mx, my) = (
        m_deg.cos() * 0.0 + m_deg.sin() * b,
        -m_deg.sin() * 0.0 + m_deg.cos() * b,
    );

    println!("{}", ((hx - mx).powi(2) + (hy - my).powi(2)).sqrt());
}
