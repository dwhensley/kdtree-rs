use std::cmp::PartialOrd;
use std::convert::AsMut;

#[derive(Debug, Clone)]
pub struct Node<T: PartialOrd + Clone, const N: usize> {
    location: [T; N],
    left_child: Option<Box<Self>>,
    right_child: Option<Box<Self>>,
}

pub fn build_tree<T: PartialOrd + Clone, const N: usize>(
    mut points: impl AsMut<[[T; N]]>,
    depth: usize,
) -> Option<Box<Node<T, N>>> {
    let points: &mut [[T; N]] = points.as_mut();
    if points.is_empty() {
        None
    } else {
        let axis = depth % N;
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
