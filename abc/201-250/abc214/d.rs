use proconio::input;

fn main() {
    input! {
        n: usize,
        mut uvw: [(usize, usize , u64); n-1]
    }

    uvw.sort_by(|a, b| a.2.cmp(&b.2));
    let mut uf = UnionFind::new(n);

    let mut ans = 0u64;
    for (u, v, w) in &uvw {
        let (u, v) = (u - 1, v - 1);
        ans += w * uf.size(u) as u64 * uf.size(v) as u64;
        uf.unite(u, v);
    }

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

    fn size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size[root]
    }
}
