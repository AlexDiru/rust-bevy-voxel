#[derive(Clone, Copy)]
pub struct Voxel {
    pub solid: bool
}

impl Voxel {
    pub fn new(solid: bool) -> Voxel {
        Voxel { solid }
    }
}