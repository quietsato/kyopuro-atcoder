use itertools::Itertools;
use whiteread::parse_line;

fn main() {
    let (x, y, z): (f32, f32, f32) = parse_line().unwrap();

    let a = (z / x) * y;
    println!("{}", if a.floor() == a.ceil() {
        a.floor() as u32 - 1
    } else {
        a.floor() as u32
    } );    
}
