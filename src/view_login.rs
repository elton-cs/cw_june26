use crate::constants::{
    VIEW_BUTTON_COLOR, VIEW_CONTENT_Z, VIEW_INPUT_FIELD_COLOR, VIEW_LOGIN_BUTTON_FONT_SIZE,
    VIEW_LOGIN_BUTTON_HEIGHT, VIEW_LOGIN_BUTTON_WIDTH, VIEW_LOGIN_ELEMENT_Y_SPACING,
    VIEW_LOGIN_INPUT_HEIGHT, VIEW_LOGIN_INPUT_WIDTH, VIEW_LOGIN_INPUT_Y_START,
    VIEW_LOGIN_PLACEHOLDER_TEXT_COLOR, VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z,
    VIEW_TEXT_COLOR, VIEW_TITLE, VIEW_TITLE_TILE_COLOR, VIEW_TITLE_TILE_SIZE,
    VIEW_TITLE_TILE_SPACING, VIEW_TITLE_Y,
};
use crate::view_ty::{
    LoginInput, ViewButton, ViewInputField, ViewInputText, ViewLoginElement, ViewTile, ViewTitle,
};
use bevy::prelude::*;

pub fn spawn_login_title(mut commands: Commands) {
    let title_chars: Vec<char> = VIEW_TITLE.to_ascii_uppercase().chars().collect();
    let tile_count = title_chars.len() as f32;

    let start_x = -((tile_count - 1.0) * VIEW_TITLE_TILE_SPACING) / 2.0;
    let text_font = TextFont::from_font_size(VIEW_TITLE_TILE_SIZE / 2.);

    commands
        .spawn((
            ViewTitle,
            ViewLoginElement,
            Transform::from_xyz(VIEW_ORIGIN_X, VIEW_TITLE_Y, VIEW_ORIGIN_Z),
            Visibility::Hidden,
        ))
        .with_children(|p1| {
            for (index, letter) in title_chars.iter().enumerate() {
                let x = start_x + index as f32 * VIEW_TITLE_TILE_SPACING;

                p1.spawn((ViewTile, Transform::default(), Visibility::default()))
                    .with_children(|p2| {
                        p2.spawn((
                            Sprite::from_color(
                                Color::from(VIEW_TITLE_TILE_COLOR),
                                Vec2::new(VIEW_TITLE_TILE_SIZE, VIEW_TITLE_TILE_SIZE),
                            ),
                            Transform::from_xyz(x, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z),
                        ));
                        p2.spawn((
                            Text2d::new(letter.to_string()),
                            TextColor(VIEW_TEXT_COLOR.into()),
                            text_font.clone(),
                            Transform::from_xyz(x, VIEW_ORIGIN_Y, VIEW_CONTENT_Z),
                        ));
                    });
            }
        });
}

pub fn spawn_login_input_field(mut commands: Commands) {
    let input_font = TextFont::from_font_size(VIEW_LOGIN_BUTTON_FONT_SIZE);

    let placeholder = "ENTER NAME".to_string();
    let placeholder_color = VIEW_LOGIN_PLACEHOLDER_TEXT_COLOR;
    let input_color = VIEW_TEXT_COLOR;

    commands
        .spawn((
            ViewInputField,
            LoginInput {
                placeholder: placeholder.clone(),
                input: String::new(),
                placeholder_color: placeholder_color.into(),
                input_color: input_color.into(),
            },
            ViewLoginElement,
            Transform::from_xyz(VIEW_ORIGIN_X, VIEW_LOGIN_INPUT_Y_START, VIEW_ORIGIN_Z),
            Visibility::Hidden,
        ))
        .with_children(|p1| {
            p1.spawn((
                Sprite::from_color(
                    Color::from(VIEW_INPUT_FIELD_COLOR),
                    Vec2::new(VIEW_LOGIN_INPUT_WIDTH, VIEW_LOGIN_INPUT_HEIGHT),
                ),
                Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z),
            ));
            p1.spawn((
                ViewInputText,
                Text2d::new(placeholder.clone()),
                TextColor(placeholder_color.into()),
                input_font.clone(),
                Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_CONTENT_Z),
            ));
        });
}

pub fn spawn_login_join_button(mut commands: Commands) {
    let join_font = TextFont::from_font_size(VIEW_LOGIN_BUTTON_FONT_SIZE);
    let join_y = VIEW_LOGIN_INPUT_Y_START - VIEW_LOGIN_ELEMENT_Y_SPACING;

    commands
        .spawn((
            ViewButton,
            ViewLoginElement,
            Transform::from_xyz(VIEW_ORIGIN_X, join_y, VIEW_ORIGIN_Z),
            Visibility::Hidden,
        ))
        .with_children(|p1| {
            p1.spawn((
                Sprite::from_color(
                    VIEW_BUTTON_COLOR,
                    Vec2::new(VIEW_LOGIN_BUTTON_WIDTH, VIEW_LOGIN_BUTTON_HEIGHT),
                ),
                Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z),
            ));
            p1.spawn((
                Text2d::new("JOIN"),
                TextColor(VIEW_TEXT_COLOR.into()),
                join_font,
                Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_CONTENT_Z),
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
