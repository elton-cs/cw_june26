use bevy::color::palettes::{
    css::{BLACK, WHITE},
    tailwind::*,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct ViewLoginElement;

#[derive(Component)]
pub struct LoginTitle;

#[derive(Component)]
pub struct LoginTile;

#[derive(Component)]
pub struct LoginInputField;

#[derive(Component)]
pub struct LoginInputText;

#[derive(Component)]
pub struct LoginJoinButton;

#[allow(dead_code)]
#[derive(Component)]
pub struct LoginInput {
    pub placeholder: String,
    pub input: String,
    pub placeholder_color: Color,
    pub input_color: Color,
}

const TITLE: &str = "CROSSWORDLE";
const TILE_SIZE: f32 = 80.;
const TILE_SPACING: f32 = TILE_SIZE * 1.1;

const LOGIN_BUTTON_WIDTH: f32 = 200.0;
const LOGIN_BUTTON_HEIGHT: f32 = 60.0;
const LOGIN_INPUT_WIDTH: f32 = 600.0;
const LOGIN_INPUT_HEIGHT: f32 = 60.0;
const LOGIN_INPUT_Y_START: f32 = -140.0;
const LOGIN_ELEMENT_Y_SPACING: f32 = 80.0;
const LOGIN_BUTTON_FONT_SIZE: f32 = 28.;

#[allow(dead_code)]
const LOGIN_INPUT_MAX_LEN: usize = 30;

pub fn spawn_login_title(mut commands: Commands) {
    let title_chars: Vec<char> = TITLE.to_ascii_uppercase().chars().collect();
    let tile_count = title_chars.len() as f32;

    let start_x = -((tile_count - 1.0) * TILE_SPACING) / 2.0;
    let text_font = TextFont::from_font_size(TILE_SIZE / 2.);

    commands
        .spawn((
            LoginTitle,
            ViewLoginElement,
            Transform::from_xyz(0.0, 100.0, 0.0),
            Visibility::Hidden,
        ))
        .with_children(|p1| {
            for (index, letter) in title_chars.iter().enumerate() {
                let x = start_x + index as f32 * TILE_SPACING;

                p1.spawn((LoginTile, Transform::default(), Visibility::default()))
                    .with_children(|p2| {
                        p2.spawn((
                            Sprite::from_color(SLATE_400, Vec2::new(TILE_SIZE, TILE_SIZE)),
                            Transform::from_xyz(x, 0.0, 0.0),
                            Visibility::default(),
                        ));
                        p2.spawn((
                            Text2d::new(letter.to_string()),
                            TextColor::BLACK,
                            text_font.clone(),
                            Transform::from_xyz(x, 0.0, 1.0),
                            Visibility::default(),
                        ));
                    });
            }
        });
}

pub fn spawn_login_input_field(mut commands: Commands) {
    let input_font = TextFont::from_font_size(LOGIN_BUTTON_FONT_SIZE);

    let placeholder = "ENTER NAME".to_string();
    let placeholder_color = WHITE.into();
    let input_color = BLACK.into();

    commands
        .spawn((
            LoginInputField,
            LoginInput {
                placeholder: placeholder.clone(),
                input: String::new(),
                placeholder_color,
                input_color,
            },
            ViewLoginElement,
            Transform::from_xyz(0.0, LOGIN_INPUT_Y_START, 0.0),
            Visibility::Hidden,
        ))
        .with_children(|p1| {
            p1.spawn((
                Sprite::from_color(SLATE_200, Vec2::new(LOGIN_INPUT_WIDTH, LOGIN_INPUT_HEIGHT)),
                Transform::from_xyz(0.0, 0.0, 0.0),
                Visibility::default(),
            ));
            p1.spawn((
                LoginInputText,
                Text2d::new(placeholder.clone()),
                TextColor(placeholder_color),
                input_font.clone(),
                Transform::from_xyz(0.0, 0.0, 1.0),
                Visibility::default(),
            ));
        });
}

pub fn spawn_login_join_button(mut commands: Commands) {
    let join_font = TextFont::from_font_size(LOGIN_BUTTON_FONT_SIZE);
    let join_y = LOGIN_INPUT_Y_START - LOGIN_ELEMENT_Y_SPACING;

    commands
        .spawn((
            LoginJoinButton,
            ViewLoginElement,
            Transform::from_xyz(0.0, join_y, 0.0),
            Visibility::Hidden,
        ))
        .with_children(|p1| {
            p1.spawn((
                Sprite::from_color(
                    SLATE_200,
                    Vec2::new(LOGIN_BUTTON_WIDTH, LOGIN_BUTTON_HEIGHT),
                ),
                Transform::from_xyz(0.0, 0.0, 0.0),
                Visibility::default(),
            ));
            p1.spawn((
                Text2d::new("JOIN"),
                TextColor::BLACK,
                join_font,
                Transform::from_xyz(0.0, 0.0, 1.0),
                Visibility::default(),
            ));
        });
}

pub fn show_login_elements(mut elements: Query<&mut Visibility, With<ViewLoginElement>>) {
    for mut visibility in elements.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

pub fn hide_login_elements(mut elements: Query<&mut Visibility, With<ViewLoginElement>>) {
    for mut visibility in elements.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}
