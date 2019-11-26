use super::*;

#[test]
fn edge_is_2_and_len_is_4_if_create_with_edge_size_2() {
    let chunk = Chunk::<f32>::with_edge_size(2);
    assert_eq!(2, chunk.edge_size);
    assert_eq!(4, chunk.height_map.len());
}

#[test]
fn edge_is_4_and_len_is_16_if_create_with_edge_size_5() {
    let chunk = Chunk::<f32>::with_edge_size(5);
    assert_eq!(4, chunk.edge_size);
    assert_eq!(16, chunk.height_map.len());
}

#[test]
fn diamond_corners_returns_center_if_0_diagonal_passed() {
    let mut chunk = Chunk::<f32>::with_edge_size(5);
    let center = chunk.edge_size() / 2;
    let expected_value = 1f32;
    let expected = [Some(expected_value); 4];

    *chunk.at_mut(center, center) = expected_value;

    assert_eq!(expected, chunk.diamond_corners(center, center, 0));
}

#[test]
fn diamond_corners_returns_closest_neighbours_if_2_diagonal_passed() {
    let mut chunk = Chunk::<f32>::with_edge_size(5);
    let center = chunk.edge_size() / 2;
    let expected = [Some(1f32), Some(2f32), Some(3f32), Some(4f32)];
    let diagonal = 2;
    let half = diagonal / 2;

    *chunk.at_mut(center, center - half) = expected[0].unwrap();
    *chunk.at_mut(center + half, center) = expected[1].unwrap();
    *chunk.at_mut(center, center + half) = expected[2].unwrap();
    *chunk.at_mut(center - half, center) = expected[3].unwrap();

    assert_eq!(expected, chunk.diamond_corners(center, center, 2));
}

#[test]
fn square_corners_returns_center_if_0_edge_passed() {
    let mut chunk = Chunk::<f32>::with_edge_size(5);
    let center = chunk.edge_size() / 2;
    let expected_value = 1f32;
    let expected = [Some(expected_value); 4];

    *chunk.at_mut(center, center) = expected_value;

    assert_eq!(expected, chunk.square_corners(center, center, 0));
}

#[test]
fn square_corners_returns_closest_neighbours_if_2_diagonal_passed() {
    let mut chunk = Chunk::<f32>::with_edge_size(5);
    let center = chunk.edge_size() / 2;
    let expected = [Some(1f32), Some(2f32), Some(3f32), Some(4f32)];
    let diagonal = 2;
    let half = diagonal / 2;

    *chunk.at_mut(center - half, center - half) = expected[0].unwrap();
    *chunk.at_mut(center + half, center - half) = expected[1].unwrap();
    *chunk.at_mut(center + half, center + half) = expected[2].unwrap();
    *chunk.at_mut(center - half, center + half) = expected[3].unwrap();

    assert_eq!(expected, chunk.square_corners(center, center, 2));
}

#[test]
fn eight_returns_closest_neighbours_if_2_diagonal_passed() {
    let mut chunk = Chunk::<f32>::with_edge_size(5);
    let center = chunk.edge_size() / 2;
    let expected = [
        Some(1f32),
        Some(2f32),
        Some(3f32),
        Some(4f32),
        Some(5f32),
        Some(6f32),
        Some(7f32),
        Some(8f32),
    ];
    let diagonal = 2;
    let half = diagonal / 2;

    *chunk.at_mut(center - half, center - half) = expected[0].unwrap();
    *chunk.at_mut(center, center - half) = expected[1].unwrap();
    *chunk.at_mut(center + half, center - half) = expected[2].unwrap();
    *chunk.at_mut(center + half, center) = expected[3].unwrap();
    *chunk.at_mut(center + half, center + half) = expected[4].unwrap();
    *chunk.at_mut(center, center + half) = expected[5].unwrap();
    *chunk.at_mut(center - half, center + half) = expected[6].unwrap();
    *chunk.at_mut(center - half, center) = expected[7].unwrap();

    assert_eq!(expected, chunk.eight_neighbours(center, center, 2));
}
