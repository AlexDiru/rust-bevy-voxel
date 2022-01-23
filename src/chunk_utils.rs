use crate::IVec3;

pub fn xyz_to_voxel_index(xyz: &IVec3, chunk_size: &IVec3) -> usize {
    (xyz.x + (xyz.y * chunk_size.x) + (xyz.z * chunk_size.x * chunk_size.y)) as usize
}

pub fn voxel_index_to_xyz(idx: i32, chunk_size: &IVec3) -> IVec3 {
    let z = idx / (chunk_size.x * chunk_size.y);
    let idx_yx = idx - (z * chunk_size.x * chunk_size.y);
    let y = idx_yx / chunk_size.x;
    let x = idx_yx % chunk_size.x;
    return IVec3::new(x, y, z)
}

#[cfg(test)]
mod tests {
    use crate::chunk_utils::voxel_index_to_xyz;
    use crate::IVec3;

    #[test]
    fn voxel_index_to_xyz_test() {
        let size = IVec3::new(32, 32, 32);
        assert_eq!(voxel_index_to_xyz(0, &size), IVec3::new(0, 0, 0));
        assert_eq!(voxel_index_to_xyz(1, &size), IVec3::new(1, 0, 0));
        assert_eq!(voxel_index_to_xyz(31, &size), IVec3::new(31, 0, 0));
        assert_eq!(voxel_index_to_xyz(32, &size), IVec3::new(0, 1, 0));
        assert_eq!(voxel_index_to_xyz(63, &size), IVec3::new(31, 1, 0));
        assert_eq!(voxel_index_to_xyz(64, &size), IVec3::new(0, 2, 0));
        assert_eq!(voxel_index_to_xyz(32 * 32, &size), IVec3::new(0, 0, 1));
        assert_eq!(voxel_index_to_xyz(32 * 32 + 35, &size), IVec3::new(3, 1, 1));
    }
}