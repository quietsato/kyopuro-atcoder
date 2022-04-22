use std::collections::{BTreeMap, BinaryHeap};

use proconio::input;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RectType {
    Chocolate,
    Box,
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
        b: [u64; n],
        c: [u64; m],
        d: [u64; m]
    }

    let mut heap = BinaryHeap::new();

    for (&a, &b) in a.iter().zip(b.iter()) {
        heap.push((a, RectType::Chocolate, b));
    }
    for (&c, &d) in c.iter().zip(d.iter()) {
        heap.push((c, RectType::Box, d));
    }

    let mut available_boxes = BTreeMap::new();
    while let Some((_, ty, h)) = heap.pop() {
        match ty {
            RectType::Box => {
                if let Some(val) = available_boxes.get_mut(&h) {
                    *val += 1;
                } else {
                    available_boxes.insert(h, 1);
                }
            }
            RectType::Chocolate => {
                if let Some((key, _)) = available_boxes.range(h..).next() {
                    let key = *key;
                    let val = available_boxes.get_mut(&key).unwrap();
                    *val -= 1;
                    if *val == 0 {
                        available_boxes.remove(&key);
                    }
                } else {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
