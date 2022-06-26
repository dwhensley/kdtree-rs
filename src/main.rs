use kdtree::build_tree;

fn main() {
    let mut integer_points_2d: Vec<[usize; 2]> =
        vec![[7, 2], [5, 4], [9, 6], [4, 7], [8, 1], [2, 3]];
    let kd_tree_ip2d = build_tree(&mut integer_points_2d, 0);
    println!(
        "\nKD Tree (2D Integer Points)\n\nPoints:\n{:?}\n\nTree:\n{:?}",
        &integer_points_2d, &kd_tree_ip2d
    );

    let mut f32_points_3d: Vec<[f32; 3]> = vec![
        [7.5, 2.1, 3.3],
        [5.2, 4.7, 4.4],
        [9.9, 6.2, 4.9],
        [4.7, 7.7, 8.8],
        [8.3, 1.9, 5.8],
        [2.7, 3.3, 6.8],
    ];
    let kd_tree_fp3d = build_tree(&mut f32_points_3d, 0);
    println!(
        "\nKD Tree (3D Real Points)\n\nPoints:\n{:?}\n\nTree:\n{:?}",
        &f32_points_3d, &kd_tree_fp3d
    );
}
