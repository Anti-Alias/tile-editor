use tile_editor::voxel::{VoxelMap, Coords, Selection, RawChunk, Size, Slot};

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