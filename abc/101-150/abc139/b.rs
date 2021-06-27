fn main() {
    let (n, m) = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let list = input
            .split_whitespace()
            .map(|str| str.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        (list[0], list[1])
    };

    let answer = {
        let mut answer = 0;
        let mut remaining = m;
        loop {
            if remaining <= 1 {
                break;
            }
            remaining -= n;
            answer += 1;
            if remaining >= 1 {
                remaining += 1
            }
        }
        answer
    };

    println!("{}", answer);
}
