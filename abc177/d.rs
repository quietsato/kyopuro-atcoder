#[rustfmt::skip]
#[allow(unused_imports)]
use {
    itertools,
    whiteread::parse_line
};

use std::*;

struct UnionFind {
    parent: Vec<usize>,
    size: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            size,
        }
    }

    fn unite(&mut self, a: usize, b: usize) -> usize {
        let (mut a, mut b) = (a, b);
        if a > b {
            mem::swap(&mut a, &mut b);
        }

        let ra = self.root(a);
        let rb = self.root(b);

        if ra == rb {
            ra
        } else {
            self.parent[rb] = ra;
            ra
        }
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

#[cfg(test)]
mod test {
    use super::UnionFind;

    #[test]
    fn union_find() {
        let mut uf = UnionFind::new(3);

        assert_eq!(uf.root(0), 0);
        assert_eq!(uf.root(1), 1);
        assert_eq!(uf.root(2), 2);

        uf.unite(0, 1);
        assert_eq!(uf.root(0), 0);
        assert_eq!(uf.root(1), 0);

        uf.unite(2, 1);
        assert_eq!(uf.root(0), 0);
        assert_eq!(uf.root(1), 0);
        assert_eq!(uf.root(2), 0);
    }
}

fn main() {
    let (n, m): (usize, usize) = parse_line().unwrap();

    let mut uf = UnionFind::new(n);

    (0..m).into_iter().for_each(|_| {
        let (mut a, mut b): (usize, usize) = parse_line().unwrap();
        a -= 1;
        b -= 1;

        uf.unite(a, b);
    });

    let ans = {
        let mut cnt = vec![0; n];
        for i in 0..n {
            cnt[uf.root(i)] += 1;
        }
        cnt.into_iter().max().unwrap()
    };

    println!("{}", ans);
}
