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

pub fn exit_home() {}

pub fn enter_login() {}
pub fn exit_login() {}
pub fn enter_wordle() {}
pub fn exit_wordle() {}
pub fn enter_crossword() {}
pub fn exit_crossword() {}
pub fn enter_crafting() {}
pub fn exit_crafting() {}
