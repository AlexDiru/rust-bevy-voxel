#[derive(Clone, Copy)]
pub struct Voxel {
    pub solid: bool
}

impl Voxel {
    pub fn new(solid: bool) -> Voxel {
        Voxel { solid }
    }
}

pub struct Chunk {
    voxels: [[[Voxel; 32]; 32]; 32]
}

impl Chunk {
    pub fn all_solid() -> Chunk {
        let voxels = [[[Voxel::new(true); 32]; 32]; 32];

        Chunk { voxels }
    }

    pub fn doughnut() -> Chunk {
        let mut voxels = [[[Voxel::new(false); 32]; 32]; 32];

        for i in 1..3 {
            voxels[i][0][0].solid = true;
            voxels[i][0][1].solid = true;
            voxels[i][0][2].solid = true;
            voxels[i][1][2].solid = true;
            voxels[i][2][2].solid = true;
            voxels[i][2][1].solid = true;
            voxels[i][2][0].solid = true;
            voxels[i][1][0].solid = true;
        }

        Chunk {voxels}
    }

    pub fn three_mesh() -> Chunk {
        let mut voxels = [[[Voxel::new(false); 32]; 32]; 32];

        voxels[1][0][2].solid = true;
        voxels[4][2][2].solid = true;
        voxels[7][2][0].solid = true;

        Chunk {voxels}
    }

    pub fn sphere() -> Chunk {
        let mut voxels = [[[Voxel::new(false); 32]; 32]; 32];

        for n in 0..(32 * 32 * 32) {
            /**
            x = i % max_x
            y = ( i / max_x ) % max_y
            z = i / ( max_x * max_y ) */
            let x = n % 32;
            let y = (n / 32) % 32;
            let z = n / (32 * 32);

            let sx = x as f32 - 16.0;
            let sy = y as f32 - 16.0;
            let sz = z as f32 - 16.0;
            voxels[z][y][x].solid = (sz * sz + sy * sy + sx * sx) < (15.0 * 15.0);
        }

        Chunk { voxels }
    }

    pub fn get_voxel(&self, index: &(usize, usize, usize)) -> &Voxel {
        return &self.voxels[index.2][index.1][index.0];
    }

    pub fn in_bounds(vx: &(usize, usize, usize)) -> bool {
        vx.2 >= 0 && vx.2 <= 32 && vx.1 >= 0 && vx.1 <= 32 && vx.0 >= 0 && vx.0 <= 32
    }
}




