mod constants;
mod game_states;
mod setup;
mod view_crafting;
mod view_crossword;
mod view_home;
mod view_login;
mod view_ty;
mod view_wordle;

use crate::game_states::{View, system_cycle_view_state};
use crate::setup::setup_system;
use crate::view_crafting::{
    hide_crafting_elements, show_crafting_elements, spawn_crafting_frag_display,
    spawn_crafting_rune_display,
};
use crate::view_crossword::{
    hide_crossword_elements, show_crossword_elements, spawn_crossword_frag_display,
    spawn_crossword_rune_display,
};
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
            spawn_crafting_frag_display,
            spawn_crafting_rune_display,
            spawn_crossword_frag_display,
            spawn_crossword_rune_display,
        ),
    );

    app.add_systems(OnEnter(View::Home), show_home_elements);
    app.add_systems(OnExit(View::Home), hide_home_elements);
    app.add_systems(OnEnter(View::Login), show_login_elements);
    app.add_systems(OnExit(View::Login), hide_login_elements);
    app.add_systems(OnEnter(View::Wordle), show_wordle_elements);
    app.add_systems(OnExit(View::Wordle), hide_wordle_elements);
    app.add_systems(OnEnter(View::Crossword), show_crossword_elements);
    app.add_systems(OnExit(View::Crossword), hide_crossword_elements);
    app.add_systems(OnEnter(View::Crafting), show_crafting_elements);
    app.add_systems(OnExit(View::Crafting), hide_crafting_elements);

    app.add_systems(Update, system_cycle_view_state);
}
