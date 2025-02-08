use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    // 鳩のいる位置
    let mut pos = (0..n).collect::<Vec<_>>();
    // 巣iにいる鳩の数
    let mut cnt = vec![1; n];
    // 現状2匹以上いる巣の数
    let mut ans = 0;

    for _ in 0..q {
        input! {
            t: usize,
        };

        if t == 1 {
            input! {
                p: Usize1,
                h: Usize1,
            };

            cnt[pos[p]] -= 1;
            // 鳩の移動により、移動元の巣が1匹だけになったら答えから1を引く
            if cnt[pos[p]] == 1 {
                ans -= 1;
            }

            pos[p] = h;

            // 鳩の移動により、移動先の巣が2匹になったら答えに1を足す
            cnt[h] += 1;
            if cnt[h] == 2 {
                ans += 1;
            }
        } else if t == 2 {
            println!("{}", ans);
        }
    }
}
