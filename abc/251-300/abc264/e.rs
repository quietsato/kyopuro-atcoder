use std::collections::BTreeSet;

use std::iter::FromIterator;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        _m: usize,
        e: usize,
        uv: [(Usize1, Usize1); e],
        q: usize,
        mut x: [Usize1; q]
    };
    x.reverse();
    let x = x;

    let mut uf = UnionFind::new(n + 1);

    {
        let x = BTreeSet::from_iter(&x);
        for (i, &(u, v)) in uv.iter().enumerate() {
            if x.contains(&i) {
                continue;
            }
            let (u, v) = (u.min(n), v.min(n));
            uf.unite(u, v);
        }
    };

    let mut ans = vec![];
    for &x in &x {
        ans.push(uf.size(n));
        let (u, v) = uv[x];
        let (u, v) = (u.min(n), v.min(n));
        uf.unite(u, v);
    }
    ans.reverse();

    println!("{}", ans.iter().map(|a| a - 1).join("\n"));
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

    fn size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size[root]
    }
}
