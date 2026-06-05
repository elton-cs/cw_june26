mod game_states;
mod setup;
mod view_home;

use crate::game_states::{
    View, enter_crafting, enter_crossword, enter_login, enter_wordle, exit_crafting,
    exit_crossword, exit_home, exit_login, exit_wordle,
};
use crate::setup::setup_system;
use crate::view_home::{spawn_exit_button, spawn_play_button, spawn_title};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins);

    app.init_state::<View>();

    app.add_systems(Startup, setup_system);
    app.add_systems(
        OnEnter(View::Home),
        (spawn_title, spawn_play_button, spawn_exit_button),
    );
    app.add_systems(OnExit(View::Home), exit_home);
    app.add_systems(OnEnter(View::Login), enter_login);
    app.add_systems(OnExit(View::Login), exit_login);
    app.add_systems(OnEnter(View::Wordle), enter_wordle);
    app.add_systems(OnExit(View::Wordle), exit_wordle);
    app.add_systems(OnEnter(View::Crossword), enter_crossword);
    app.add_systems(OnExit(View::Crossword), exit_crossword);
    app.add_systems(OnEnter(View::Crafting), enter_crafting);
    app.add_systems(OnExit(View::Crafting), exit_crafting);
}
