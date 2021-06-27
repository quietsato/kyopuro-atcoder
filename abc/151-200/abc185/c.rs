use whiteread::parse_line;

fn main() {
    let l: u128 = (parse_line::<u64>().unwrap() - 1).into();

    let ans: u128 = {
        // 切断場所候補`L-1`個の中から11個を選ぶ組み合わせの総数
        (1..=11).map(|i| l - i + 1).product::<u128>() / (1..=11).product::<u128>()
    };

    println!("{}", ans);
}

