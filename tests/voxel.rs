use tile_editor::voxel::{VoxelMap, Coords, Selection, Size, Slot, VoxelId};

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
    let slot = map.slot_at(&Coords::new(10, -11, 12));
    let expected = Coords::new(10, -11, 12);
    let actual = slot.coords();
    assert_eq!(expected, actual);
}

#[test]
fn set_voxel() {
    let occupied_coords = [
        Coords::new(0, 0, 0),
        Coords::new(-100, -101, -102),
        Coords::new(-32, 32, -32),
        Coords::new(31, -31, 31),
    ];
    let unoccupied_coords = [
        Coords::new(1, 0, 0),
        Coords::new(-99, -101, -102),
        Coords::new(-249, 251, -252),
        Coords::new(349, -351, 352),
    ];
    let mut map = VoxelMap::new(Size::new(32, 32, 32));

    // Checks that all coords specified are occupied
    for coords in &occupied_coords {
        let mut slot = map.slot_at(coords);
        let expected = VoxelId::Index { set_idx: 0, voxel_idx: 0 };
        slot.set(0, expected);
        let actual = slot.get(0);
        assert_eq!(expected, actual);
    }

    // Checks that all unoccupied coords are unoccupied
    for coords in &unoccupied_coords {
        let mut slot = map.slot_at(coords);
        let expected = VoxelId::Empty;
        slot.set(0, expected);
        let actual = slot.get(0);
        assert_eq!(expected, actual);
    }

    // Checks that newly unoccupied coord is now unoccupied
    let coords = Coords::new(-100, -101, -102);
    let mut slot = map.slot_at(&coords);
    let expected_old = VoxelId::Index { set_idx: 0, voxel_idx: 0 };
    let expected_new = VoxelId::Empty;
    let actual_old = slot.get(0);
    assert_eq!(expected_old, actual_old);
    slot.set(0, expected_new);
    let actual_new = slot.get(0);
    assert_eq!(expected_new, actual_new);
}