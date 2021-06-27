use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: usize,
        a: [i64; n]
    }

    let map = {
        let mut map = vec![];

        let (mut mv, mut mv_max) = (0, 0);
        for a in a {
            mv += a;
            mv_max = mv_max.max(mv);

            map.push((mv, mv_max));
        }

        map
    };

    let (mut pos, mut pos_max) = (0, 0);
    for (mv, mv_max) in map {
        pos_max = pos_max.max(pos + mv_max);
        pos += mv;
    }

    println!("{}", pos_max);

    Ok(())
}

