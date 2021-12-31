use bevy::prelude::{Input, KeyCode, Query, Res};
use bevy_fly_camera::FlyCamera;

// Press "T" to toggle keyboard+mouse control over the camera
pub fn mouse_toggle(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut FlyCamera>,
) {
    for mut options in query.iter_mut() {
        if input.just_pressed(KeyCode::T) {
            println!("Toggled FlyCamera enabled!");
            options.enabled = !options.enabled;
        }
    }
}