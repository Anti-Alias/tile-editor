use tile_editor::voxel::{VoxelMap, Coords, Selection, Size, Slot};

#[test]
fn slot_iterator() {
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

#[test]
fn slot_at() {
    let mut map = VoxelMap::new(Size::new(32, 32, 32));
    let slot = map.slot_at(Coords::new(10, -11, 12));
    let expected = Coords::new(10, -11, 12);
    let actual = slot.coords();
    assert_eq!(expected, actual);
}
