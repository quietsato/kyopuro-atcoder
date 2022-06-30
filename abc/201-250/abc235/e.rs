use itertools::Itertools;
use proconio::{input, marker::Usize1};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EdgeType {
    G,
    Query(usize),
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abc: [(Usize1, Usize1, u64); m],
        uvw: [(Usize1, Usize1, u64); q]
    }

    let mut edges: Vec<(u64, usize, usize, EdgeType)> = abc
        .iter()
        .map(|&(a, b, c)| (c, a, b, EdgeType::G))
        .chain(
            uvw.iter()
                .enumerate()
                .map(|(i, &(u, v, w))| (w, u, v, EdgeType::Query(i))),
        )
        .collect_vec();
    edges.sort_unstable();

    let mut uf = UnionFind::new(n);
    let mut ans = vec![false; q];
    for (_, a, b, ty) in edges {
        if let EdgeType::Query(i) = ty {
            ans[i] = uf.root(a) != uf.root(b);
        } else if uf.root(a) != uf.root(b) {
            uf.unite(a, b);
        }
    }

    println!(
        "{}",
        ans.into_iter()
            .map(|b| if b { "Yes" } else { "No" })
            .collect_vec()
            .join("\n")
    );
}

// -----------------------------------------------------------------------------
// Union-Find
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
