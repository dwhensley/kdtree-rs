#![no_std]

use core::cmp::PartialOrd;
use core::convert::AsMut;

extern crate alloc;
use alloc::boxed::Box;

pub mod arena;

#[derive(Debug, Clone)]
pub struct Node<T: PartialOrd + Clone, const K: usize> {
    location: [T; K],
    left_child: Option<Box<Self>>,
    right_child: Option<Box<Self>>,
}

pub fn build_tree<T: PartialOrd + Clone, const K: usize>(
    mut points: impl AsMut<[[T; K]]>,
    depth: usize,
) -> Option<Box<Node<T, K>>> {
    let points: &mut [[T; K]] = points.as_mut();
    if points.is_empty() {
        None
    } else {
        let axis = depth % K;
        points
            .sort_unstable_by(|a, b| a[axis].partial_cmp(&b[axis]).expect("`partial_cmp` failed"));
        let median_loc = points.len() / 2;
        Some(Box::new(Node {
            location: points[median_loc].clone(),
            left_child: build_tree(&mut points[0..median_loc], depth + 1),
            right_child: build_tree(&mut points[median_loc + 1..], depth + 1),
        }))
    }
}
