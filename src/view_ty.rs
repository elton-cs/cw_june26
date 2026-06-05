use bevy::prelude::Color;
use bevy::prelude::Component;

#[derive(Component)]
pub struct ViewHomeElement;

#[derive(Component)]
pub struct ViewLoginElement;

#[derive(Component)]
pub struct ViewWordleElement;

#[derive(Component)]
pub struct ViewCraftingElement;

#[derive(Component)]
pub struct ViewCrosswordElement;

#[derive(Component)]
pub struct ViewTitle;

#[derive(Component)]
pub struct ViewTile;

#[derive(Component)]
pub struct ViewButton;

#[derive(Component)]
pub struct ViewInputField;

#[derive(Component)]
pub struct ViewInputText;

#[allow(dead_code)]
#[derive(Component)]
pub struct LoginInput {
    pub placeholder: String,
    pub input: String,
    pub placeholder_color: Color,
    pub input_color: Color,
}

#[derive(Clone, Copy)]
pub enum DisplaySide {
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum DisplayTileColor {
    Purple,
    Blue,
}
