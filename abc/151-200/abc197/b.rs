#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};


fn main() {
    let (h, w, y, x): (usize, usize, usize, usize) = parse_line().unwrap();
    let map = {
        let mut map: Vec<Vec<char>> = vec![];

        for _ in 0..h {
            let s = parse_line::<String>()
                .unwrap()
                .chars()
                .collect::<Vec<char>>();
            map.push(s);
        }

        map
    };

    let ans = {
        let mut ans = -3;

        let diff_list  = [
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
        ];

        for diff in diff_list.iter() {
            let mut current_position = (x - 1, y - 1);

            while map[current_position.1][current_position.0] == '.' {
                ans += 1;

                // check overflow
                if (diff.0 == -1 && current_position.0 == 0)
                    || (diff.0 == 1 && current_position.0 == w - 1)
                    || (diff.1 == -1 && current_position.1 == 0)
                    || (diff.1 == 1 && current_position.1 == h - 1)
                {
                    break;
                }

                current_position.0 = (current_position.0 as i64 + diff.0) as usize;
                current_position.1 = (current_position.1 as i64 + diff.1) as usize;
            }
        }

        ans
    };

    println!("{}", ans);
}
