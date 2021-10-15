use tile_editor::voxel::{VoxelMap, Coords, Selection, Chunk};

#[test]
fn new() {
    let mut map = VoxelMap::new(32, 32, 32);
    let chunks = map
        .select_chunks(Selection::new(0, 0, 0, 1, 1, 1))
        .collect::<Vec<&mut Chunk>>();
    assert_eq!(8, chunks.len());
    let chunks = map
        .select_chunks(Selection::new(1, 1, 1, 2, 2, 2))
        .collect::<Vec<&mut Chunk>>();
    assert_eq!(8, chunks.len());
    let chunks = map
        .select_chunks(Selection::new(-1, -1, -1, -2, -2, -2))
        .collect::<Vec<&mut Chunk>>();
    assert_eq!(8, chunks.len());
    let chunks = map
        .select_chunks(Selection::new(-1, -1, -1, -3, -2, -2))
        .collect::<Vec<&mut Chunk>>();
    assert_eq!(12, chunks.len());
}