use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m]
    }

    let mut con = vec![vec![]; n];
    for &(a, b) in &ab {
        con[a].push(b);
        con[b].push(a);
    }
    let mut uf = UnionFind::new(n);

    let mut ans = vec![0];
    for i in (0..n).rev() {
        for &to in &con[i] {
            if to < i {
                continue;
            }
            uf.unite(i, to);
        }
        ans.push(uf.groups - i);
    }

    ans.reverse();
    for ans in ans.iter().skip(1) {
        println!("{}", ans);
    }
}

struct UnionFind {
    parent: Vec<usize>,
    groups: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            groups: size,
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
            self.groups -= 1;
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
