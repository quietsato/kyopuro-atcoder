use std::mem;
use proconio::input;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
        }
    }
    fn unite(&mut self, a: usize, b: usize) -> usize {
        let (mut a, mut b) = (a, b);
        if a > b {
            mem::swap(&mut a, &mut b);
        }

        let (ra, rb) = (self.root(a), self.root(b));

        if ra != rb {
            self.parent[rb] = ra;
        }
        ra
    }

    fn root(&mut self, i: usize) -> usize {
        if i == self.parent[i] {
            i
        } else {
            self.parent[i] = self.root(self.parent[i]);
            self.parent[i]
        }
    }
}

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let mut uf = UnionFind::new(2 * 100_000);

    let mut ans = 0;
    for l in 0..=n / 2 {
        let r = n - l - 1;

        if uf.root(a[l] as usize - 1) != uf.root(a[r] as usize - 1) {
            uf.unite(a[l] as usize - 1, a[r] as usize - 1);
            ans += 1;
        }
    }

    println!("{}", ans);
}
