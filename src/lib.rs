mod setup;

use crate::setup::setup_system;
use bevy::prelude::*;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup_system);
}
