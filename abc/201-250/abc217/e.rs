use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use proconio::input;

fn main() {
    input! {
        q: usize
    }

    let mut queue = VecDeque::new();
    let mut heap = BinaryHeap::new();

    for _ in 0..q {
        input! {c: char}
        match c {
            '1' => {
                input! {x: u64}
                queue.push_back(x);
            }
            '2' => {
                println!(
                    "{}",
                    heap.pop()
                        .map_or_else(|| { queue.pop_front().unwrap() }, |Reverse(h)| h)
                );
            }
            '3' => {
                while let Some(q) = queue.pop_back() {
                    heap.push(Reverse(q));
                }
            }
            _ => unreachable!(),
        }
    }
}
