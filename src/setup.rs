use crate::constants::VIEW_BACKGROUND_COLOR;
use bevy::prelude::*;

pub fn setup_system(mut cmd: Commands) {
    cmd.insert_resource(ClearColor(VIEW_BACKGROUND_COLOR.into()));
    cmd.spawn(Camera2d);
}
