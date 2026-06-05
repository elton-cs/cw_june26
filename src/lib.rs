mod game_states;
mod setup;
mod view_home;
mod view_login;
mod view_wordle;

use crate::game_states::{
    View, enter_crafting, enter_crossword, exit_crafting, exit_crossword, system_cycle_view_state,
};
use crate::setup::setup_system;
use crate::view_home::{
    hide_home_elements, show_home_elements, spawn_home_exit_button, spawn_home_play_button,
    spawn_home_title,
};
use crate::view_login::{
    hide_login_elements, show_login_elements, spawn_login_input_field, spawn_login_join_button,
    spawn_login_title,
};
use crate::view_wordle::{
    hide_wordle_elements, show_wordle_elements, spawn_wordle_frag_display, spawn_wordle_input,
    spawn_wordle_rune_display,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins);

    app.init_state::<View>();

    app.add_systems(
        Startup,
        (
            setup_system,
            spawn_home_title,
            spawn_home_play_button,
            spawn_home_exit_button,
            spawn_login_title,
            spawn_login_input_field,
            spawn_login_join_button,
            spawn_wordle_input,
            spawn_wordle_frag_display,
            spawn_wordle_rune_display,
        ),
    );

    app.add_systems(OnEnter(View::Home), show_home_elements);
    app.add_systems(OnExit(View::Home), hide_home_elements);
    app.add_systems(OnEnter(View::Login), show_login_elements);
    app.add_systems(OnExit(View::Login), hide_login_elements);
    app.add_systems(OnEnter(View::Wordle), show_wordle_elements);
    app.add_systems(OnExit(View::Wordle), hide_wordle_elements);

    app.add_systems(OnEnter(View::Crossword), enter_crossword);
    app.add_systems(OnExit(View::Crossword), exit_crossword);
    app.add_systems(OnEnter(View::Crafting), enter_crafting);
    app.add_systems(OnExit(View::Crafting), exit_crafting);

    app.add_systems(Update, system_cycle_view_state);
}
