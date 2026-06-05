use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum View {
    #[default]
    Home,
    Login,
    Wordle,
    Crossword,
    Crafting,
}

pub fn system_cycle_view_state(
    state: Res<State<View>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<View>>,
) {
    if key_input.just_pressed(KeyCode::ArrowLeft) {
        let next = match state.get() {
            View::Home => View::Crafting,
            View::Login => View::Home,
            View::Wordle => View::Login,
            View::Crossword => View::Wordle,
            View::Crafting => View::Crossword,
        };
        next_state.set(next);
    }

    if key_input.just_pressed(KeyCode::ArrowRight) {
        let next = match state.get() {
            View::Home => View::Login,
            View::Login => View::Wordle,
            View::Wordle => View::Crossword,
            View::Crossword => View::Crafting,
            View::Crafting => View::Home,
        };
        next_state.set(next);
    }
}
