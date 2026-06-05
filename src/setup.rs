use bevy::{color::palettes::tailwind::*, prelude::*};

pub fn setup_system(mut cmd: Commands) {
    cmd.insert_resource(ClearColor(SLATE_100.into()));
    cmd.spawn(Camera2d);
}
