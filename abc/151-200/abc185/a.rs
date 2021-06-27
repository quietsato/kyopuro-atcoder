fn main() {
    println!(
        "{}",
        whiteread::parse_line::<Vec<i32>>()
            .unwrap()
            .iter()
            .min()
            .unwrap()
    );
}
