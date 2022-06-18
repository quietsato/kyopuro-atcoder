use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        c: [u64; n]
    }

    const INF: u64 = 1 << 60;

    let mut ans = 0;

    let mut uf = UnionFind::new(n);
    for (i, &j) in p.iter().enumerate() {
        if uf.root(i) != uf.root(j) {
            uf.unite(i, j);
            continue;
        }
        let mut current = i;
        let mut cost = INF;
        loop {
            let next = p[current];
            cost = cost.min(c[current]);
            current = next;
            if current == i {
                break;
            }
        }
        ans += cost;
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
}
