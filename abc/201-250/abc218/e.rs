use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m]
    }

    let mut con = vec![vec![]; n];
    let mut uf = UnionFind::new(n);

    let mut abc = abc
        .into_iter()
        .filter_map(|(a, b, c)| {
            if c <= 0 {
                con[a].push(b);
                con[b].push(a);
                uf.unite(a, b);
                None
            } else {
                Some((a, b, c))
            }
        })
        .collect_vec();
    abc.sort_by(|(_, _, c1), (_, _, c2)| c1.cmp(c2));

    let ans: i64 = abc
        .into_iter()
        .filter_map(|(a, b, c)| {
            if uf.root(a) != uf.root(b) {
                uf.unite(a, b);
                None
            } else {
                Some(c)
            }
        })
        .sum();

    println!("{}", ans);
}

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            size: vec![1; size],
        }
    }

    fn unite(&mut self, a: usize, b: usize) -> usize {
        let (mut ra, mut rb) = (self.root(a), self.root(b));

        if ra == rb {
            return ra;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];

        ra
    }

    fn root(&mut self, i: usize) -> usize {
        let p = self.parent[i];
        if p == i {
            i
        } else {
            self.parent[i] = self.root(p);
            self.parent[i]
        }
    }
}
