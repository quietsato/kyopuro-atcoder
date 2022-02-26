use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    let mut ans = a[0];
    let mut last_mod = a[0] % n;
    let mut g = Graph::new(n);
    for i in 1..k {
        if !g.adj[last_mod].is_empty() {
            // 閉路を検出
            let mut cycle_length = 1;
            let mut c = last_mod;
            while g.adj[c][0] != last_mod {
                cycle_length += 1;
                c = g.adj[c][0];
            }
            let mut c = last_mod;
            for j in 0..cycle_length {
                ans += a[c]
                    * ((k - i) / cycle_length + if (k - i) % cycle_length > j { 1 } else { 0 });
                c = g.adj[c][0];
            }

            println!("{}", ans);
            return;
        }
        ans += a[last_mod];
        g.add_edge(last_mod, ans % n);
        last_mod = ans % n;
    }

    println!("{}", ans);
}
pub struct Graph {
    pub adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, i: usize, j: usize) {
        self.adj[i].push(j)
    }
}
