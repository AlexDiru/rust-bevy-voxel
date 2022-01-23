use crate::IVec3;

pub const CHUNK_SIZE_X: usize = 32;
pub const CHUNK_SIZE_Y: usize = 32;
pub const CHUNK_SIZE_Z: usize = 32;

pub const CHUNK_SIZE_X_I32: i32 = CHUNK_SIZE_X as i32;
pub const CHUNK_SIZE_Y_I32: i32 = CHUNK_SIZE_Y as i32;
pub(crate) const CHUNK_SIZE_Z_I32: i32 = CHUNK_SIZE_Z as i32;

pub fn xyz_to_voxel_index(x: usize, y: usize, z: usize) -> usize {
    x + (y * CHUNK_SIZE_X) + (z * CHUNK_SIZE_X * CHUNK_SIZE_Y)
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