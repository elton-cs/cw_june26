use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ViewHomeElement;

#[derive(Component)]
pub struct HomeTitle;

#[derive(Component)]
pub struct HomeTile;

#[derive(Component)]
pub struct HomeButton;

const TITLE: &str = "CROSSWORDLE";
const TILE_SIZE: f32 = 80.;
const TILE_SPACING: f32 = TILE_SIZE * 1.1;

const HOME_BUTTON_WIDTH: f32 = 200.0;
const HOME_BUTTON_HEIGHT: f32 = 60.0;
const HOME_BUTTON_Y_START: f32 = -140.0;
const HOME_BUTTON_Y_SPACING: f32 = 80.0;
const HOME_BUTTON_FONT_SIZE: f32 = 28.;

pub fn spawn_home_title(mut commands: Commands) {
    let title_chars: Vec<char> = TITLE.to_ascii_uppercase().chars().collect();
    let tile_count = title_chars.len() as f32;

    let start_x = -((tile_count - 1.0) * TILE_SPACING) / 2.0;
    let text_font = TextFont::from_font_size(TILE_SIZE / 2.);

    commands
        .spawn((
            HomeTitle,
            ViewHomeElement,
            Transform::from_xyz(0.0, 100.0, 0.0),
            Visibility::Visible,
        ))
        .with_children(|p1| {
            for (index, letter) in title_chars.iter().enumerate() {
                let x = start_x + index as f32 * TILE_SPACING;

                p1.spawn((HomeTile, Transform::default(), Visibility::default()))
                    .with_children(|p2| {
                        p2.spawn((
                            Sprite::from_color(SLATE_400, Vec2::new(TILE_SIZE, TILE_SIZE)),
                            Transform::from_xyz(x, 0.0, 0.0),
                        ));
                        p2.spawn((
                            Text2d::new(letter.to_string()),
                            TextColor::BLACK,
                            text_font.clone(),
                            Transform::from_xyz(x, 0.0, 1.0),
                        ));
                    });
            }
        });
}

pub fn spawn_home_play_button(mut commands: Commands) {
    let play_font = TextFont::from_font_size(HOME_BUTTON_FONT_SIZE);
    let play_y = HOME_BUTTON_Y_START;

    commands
        .spawn((
            HomeButton,
            ViewHomeElement,
            Transform::from_xyz(0.0, play_y, 0.0),
            Visibility::Visible,
        ))
        .with_children(|p1| {
            p1.spawn((
                Sprite::from_color(SLATE_200, Vec2::new(HOME_BUTTON_WIDTH, HOME_BUTTON_HEIGHT)),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            p1.spawn((
                Text2d::new("PLAY"),
                TextColor::BLACK,
                play_font,
                Transform::from_xyz(0.0, 0.0, 1.0),
            ));
        });
}

pub fn spawn_home_exit_button(mut commands: Commands) {
    let exit_font = TextFont::from_font_size(HOME_BUTTON_FONT_SIZE);
    let exit_y = HOME_BUTTON_Y_START - HOME_BUTTON_Y_SPACING;

    commands
        .spawn((
            HomeButton,
            ViewHomeElement,
            Transform::from_xyz(0.0, exit_y, 0.0),
            Visibility::Visible,
        ))
        .with_children(|p1| {
            p1.spawn((
                Sprite::from_color(SLATE_200, Vec2::new(HOME_BUTTON_WIDTH, HOME_BUTTON_HEIGHT)),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            p1.spawn((
                Text2d::new("EXIT"),
                TextColor::BLACK,
                exit_font,
                Transform::from_xyz(0.0, 0.0, 1.0),
            ));
        });
}

pub fn show_home_elements(mut elements: Query<&mut Visibility, With<ViewHomeElement>>) {
    for mut visibility in elements.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

pub fn hide_home_elements(mut elements: Query<&mut Visibility, With<ViewHomeElement>>) {
    for mut visibility in elements.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}
