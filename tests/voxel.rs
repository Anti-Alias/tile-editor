use tile_editor::voxel::{VoxelMap, Coords, Selection, RawChunk, Chunk, Size, Slot};

#[test]
fn test_chunk_iterator() {
    let mut map = VoxelMap::new(Size::new(32, 32, 32));
    let chunks = map
        .select_chunks(Selection::new(0, 0, 0, 1, 1, 1))
        .collect::<Vec<Chunk>>();
    assert_eq!(8, chunks.len());
    let chunks = map
        .select_chunks(Selection::new(1, 1, 1, 2, 2, 2))
        .collect::<Vec<Chunk>>();
    assert_eq!(8, chunks.len());
    let chunks = map
        .select_chunks(Selection::new(-3, -3, -3, -1, -2, -2))
        .collect::<Vec<Chunk>>();
    assert_eq!(12, chunks.len());
}

#[test]
fn test_slot_iterator() {
    let mut map = VoxelMap::new(Size::new(32, 32, 32));
    let expected = [
        Coords { x: -1, y: -1, z: -1 },
        Coords { x: 0, y: -1, z: -1 },
        Coords { x: -1, y: 0, z: -1 },
        Coords { x: 0, y: 0, z: -1 },
        Coords { x: -1, y: -1, z: 0 },
        Coords { x: 0, y: -1, z: 0 },
        Coords { x: -1, y: 0, z: 0 },
        Coords { x: 0, y: 0, z: 0 },
    ];
    let actual = map
        .select_slots(Selection::new(-1, -1, -1, 0, 0, 0))
        .map(|slot| slot.coords())
        .collect::<Vec<Coords>>();
    let actual = &actual[..];
    assert_eq!(expected, actual);
}

/*
#[test]
fn test_slot_insert() {
    let mut map = VoxelMap::new(Size::new(32, 32, 32));
    let actual = map
        .select_slots(Selection::new(-1, -1, -1, 0, 0, 0))
        .map(|slot| slot.coords())
        .collect::<Vec<Coords>>();
    let actual = &actual[..];
    assert_eq!(expected, actual);
}
 */