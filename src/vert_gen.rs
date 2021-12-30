
pub fn top_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
    [
        ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ]
}

pub fn left_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
    [
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
    ]
}

pub fn right_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
    [
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
    ]
}

pub fn bottom_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
    [
        ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
    ]
}

pub fn back_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
    [
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
    ]
}

pub fn front_plane_vertices(x_offset: f32, y_offset: f32, z_offset: f32) -> [([f32; 3], [f32; 3], [f32; 2]); 6] {
    [
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([0.0 + x_offset, 0.0 + y_offset, 0.0 + z_offset], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([0.0 + x_offset, 1.0 + y_offset, 1.0 + z_offset], [0.0, 1.0, 0.0], [1.0, 1.0]),
    ]
}