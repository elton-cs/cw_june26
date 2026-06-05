use crate::constants::{
    VIEW_CONTENT_Z, VIEW_HOME_BUTTON_FONT_SIZE, VIEW_HOME_BUTTON_HEIGHT, VIEW_HOME_BUTTON_WIDTH,
    VIEW_HOME_BUTTON_Y_SPACING, VIEW_HOME_BUTTON_Y_START, VIEW_ORIGIN_X, VIEW_ORIGIN_Y,
    VIEW_ORIGIN_Z, VIEW_TITLE, VIEW_TITLE_TILE_SIZE, VIEW_TITLE_TILE_SPACING, VIEW_TITLE_Y,
};
use crate::view_ty::{ViewButton, ViewHomeElement, ViewTile, ViewTitle};
use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

pub fn spawn_home_title(mut commands: Commands) {
    let title_chars: Vec<char> = VIEW_TITLE.to_ascii_uppercase().chars().collect();
    let tile_count = title_chars.len() as f32;

    let start_x = -((tile_count - 1.0) * VIEW_TITLE_TILE_SPACING) / 2.0;
    let text_font = TextFont::from_font_size(VIEW_TITLE_TILE_SIZE / 2.);

    commands
        .spawn((
            ViewHomeElement,
            ViewTitle,
            Transform::from_xyz(VIEW_ORIGIN_X, VIEW_TITLE_Y, VIEW_ORIGIN_Z),
            Visibility::Visible,
        ))
        .with_children(|p1| {
            for (index, letter) in title_chars.iter().enumerate() {
                let x = start_x + index as f32 * VIEW_TITLE_TILE_SPACING;

                p1.spawn((ViewTile, Transform::default(), Visibility::default()))
                    .with_children(|p2| {
                        p2.spawn((
                            Sprite::from_color(
                                SLATE_400,
                                Vec2::new(VIEW_TITLE_TILE_SIZE, VIEW_TITLE_TILE_SIZE),
                            ),
                            Transform::from_xyz(x, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z),
                        ));
                        p2.spawn((
                            Text2d::new(letter.to_string()),
                            TextColor::BLACK,
                            text_font.clone(),
                            Transform::from_xyz(x, VIEW_ORIGIN_Y, VIEW_CONTENT_Z),
                        ));
                    });
            }
        });
}

pub fn spawn_home_play_button(mut commands: Commands) {
    let play_font = TextFont::from_font_size(VIEW_HOME_BUTTON_FONT_SIZE);
    let play_y = VIEW_HOME_BUTTON_Y_START;

    commands
        .spawn((
            ViewButton,
            ViewHomeElement,
            Transform::from_xyz(VIEW_ORIGIN_X, play_y, VIEW_ORIGIN_Z),
            Visibility::Visible,
        ))
        .with_children(|p1| {
            p1.spawn((
                Sprite::from_color(
                    SLATE_200,
                    Vec2::new(VIEW_HOME_BUTTON_WIDTH, VIEW_HOME_BUTTON_HEIGHT),
                ),
                Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z),
            ));
            p1.spawn((
                Text2d::new("PLAY"),
                TextColor::BLACK,
                play_font,
                Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_CONTENT_Z),
            ));
        });
}

pub fn spawn_home_exit_button(mut commands: Commands) {
    let exit_font = TextFont::from_font_size(VIEW_HOME_BUTTON_FONT_SIZE);
    let exit_y = VIEW_HOME_BUTTON_Y_START - VIEW_HOME_BUTTON_Y_SPACING;

    commands
        .spawn((
            ViewButton,
            ViewHomeElement,
            Transform::from_xyz(VIEW_ORIGIN_X, exit_y, VIEW_ORIGIN_Z),
            Visibility::Visible,
        ))
        .with_children(|p1| {
            p1.spawn((
                Sprite::from_color(
                    SLATE_200,
                    Vec2::new(VIEW_HOME_BUTTON_WIDTH, VIEW_HOME_BUTTON_HEIGHT),
                ),
                Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z),
            ));
            p1.spawn((
                Text2d::new("EXIT"),
                TextColor::BLACK,
                exit_font,
                Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_CONTENT_Z),
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
