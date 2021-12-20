use std::{collections::BTreeSet};

use proconio::{input, marker::Usize1};

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
            std::mem::swap(&mut a, &mut b);
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
        m: usize,
        ab: [(Usize1, Usize1); m]
    }

    let mut v = vec![BTreeSet::new(); n];

    for &(a, b) in &ab {
        v[a].insert(b);
        v[b].insert(a);
    }

    if !v.iter().all(|set| set.len() <= 2) {
        println!("No");
        return;
    }

    let mut uf = UnionFind::new(n);
    for &(a, b) in &ab {
        if uf.root(a) == uf.root(b) {
            println!("No");
            return;
        }
        uf.unite(a, b);
    }

    println!("Yes");
}

