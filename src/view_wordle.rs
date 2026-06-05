use crate::constants::{
    FRAG_RUNE_COLUMNS, FRAG_RUNE_EDGE_MARGIN, FRAG_RUNE_PADDING, FRAG_RUNE_ROWS,
    FRAG_RUNE_TILE_COUNT, FRAG_RUNE_TILE_SIZE, FRAG_RUNE_TILE_SPACING, WORDLE_TILE_SIZE,
    WORDLE_TILE_SPACING, WORDLE_WORD,
};
use crate::view_ty::{DisplaySide, DisplayTileColor, ViewTile, ViewWordleElement};
use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_wordle_input(mut commands: Commands) {
    let wordle_chars: Vec<char> = WORDLE_WORD.chars().collect();
    let tile_count = wordle_chars.len();
    let start_x = -((tile_count as f32 - 1.0) * WORDLE_TILE_SPACING) / 2.0;
    let text_font = TextFont::from_font_size(WORDLE_TILE_SIZE / 2.);

    commands
        .spawn((
            ViewWordleElement,
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::Hidden,
        ))
        .with_children(|p1| {
            for (index, letter) in wordle_chars.iter().enumerate() {
                let x = start_x + index as f32 * WORDLE_TILE_SPACING;

                p1.spawn((
                    ViewTile,
                    Transform::from_xyz(x, 0.0, 0.0),
                    Visibility::default(),
                ))
                .with_children(|p2| {
                    p2.spawn((
                        Sprite::from_color(
                            GREEN_400,
                            Vec2::new(WORDLE_TILE_SIZE, WORDLE_TILE_SIZE),
                        ),
                        Transform::from_xyz(0.0, 0.0, 0.0),
                    ));
                    p2.spawn((
                        Text2d::new(letter.to_string()),
                        TextColor::BLACK,
                        text_font.clone(),
                        Transform::from_xyz(0.0, 0.0, 1.0),
                    ));
                });
            }
        });
}

pub fn spawn_wordle_frag_display(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    spawn_wordle_alphabet_display(
        &mut commands,
        &windows,
        DisplayTileColor::Purple,
        DisplaySide::Left,
    );
}

pub fn spawn_wordle_rune_display(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    spawn_wordle_alphabet_display(
        &mut commands,
        &windows,
        DisplayTileColor::Blue,
        DisplaySide::Right,
    );
}

fn spawn_wordle_alphabet_display(
    commands: &mut Commands,
    windows: &Query<&Window, With<PrimaryWindow>>,
    tile_color: DisplayTileColor,
    side: DisplaySide,
) {
    let box_width = (FRAG_RUNE_COLUMNS as f32 - 1.0) * FRAG_RUNE_TILE_SPACING
        + FRAG_RUNE_TILE_SIZE
        + FRAG_RUNE_PADDING * 2.0;
    let box_height = (FRAG_RUNE_ROWS as f32 - 1.0) * FRAG_RUNE_TILE_SPACING
        + FRAG_RUNE_TILE_SIZE
        + FRAG_RUNE_PADDING * 2.0;
    let start_x = -((FRAG_RUNE_COLUMNS as f32 - 1.0) * FRAG_RUNE_TILE_SPACING) / 2.0;
    let start_y = ((FRAG_RUNE_ROWS as f32 - 1.0) * FRAG_RUNE_TILE_SPACING) / 2.0;
    let window_width = windows
        .single()
        .map(|window| window.width())
        .unwrap_or(1280.0);
    let display_x = match side {
        DisplaySide::Left => -window_width / 2.0 + box_width / 2.0 + FRAG_RUNE_EDGE_MARGIN,
        DisplaySide::Right => window_width / 2.0 - box_width / 2.0 - FRAG_RUNE_EDGE_MARGIN,
    };
    let letter_font = TextFont::from_font_size(FRAG_RUNE_TILE_SIZE / 2.0);
    let number_font = TextFont::from_font_size(FRAG_RUNE_TILE_SIZE / 4.0);
    let tile_color = match tile_color {
        DisplayTileColor::Purple => PURPLE_300,
        DisplayTileColor::Blue => BLUE_300,
    };

    commands
        .spawn((
            ViewWordleElement,
            Transform::from_xyz(display_x, 0.0, 0.0),
            Visibility::Hidden,
        ))
        .with_children(|p1| {
            p1.spawn((
                Sprite::from_color(SLATE_200, Vec2::new(box_width, box_height)),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));

            for index in 0..FRAG_RUNE_TILE_COUNT {
                let column = index / FRAG_RUNE_ROWS;
                let row = index % FRAG_RUNE_ROWS;
                let x = start_x + column as f32 * FRAG_RUNE_TILE_SPACING;
                let y = start_y - row as f32 * FRAG_RUNE_TILE_SPACING;
                let letter = (b'A' + index as u8) as char;

                p1.spawn((
                    ViewTile,
                    Transform::from_xyz(x, y, 1.0),
                    Visibility::default(),
                ))
                .with_children(|p2| {
                    p2.spawn((
                        Sprite::from_color(
                            tile_color,
                            Vec2::new(FRAG_RUNE_TILE_SIZE, FRAG_RUNE_TILE_SIZE),
                        ),
                        Transform::from_xyz(0.0, 0.0, 0.0),
                    ));
                    p2.spawn((
                        Text2d::new(letter.to_string()),
                        TextColor::BLACK,
                        letter_font.clone(),
                        Transform::from_xyz(0.0, 0.0, 1.0),
                    ));
                    p2.spawn((
                        Text2d::new("100"),
                        TextColor::BLACK,
                        number_font.clone(),
                        Transform::from_xyz(0.0, -FRAG_RUNE_TILE_SIZE / 2.0 + 7.0, 2.0),
                    ));
                });
            }
        });
}

pub fn show_wordle_elements(mut elements: Query<&mut Visibility, With<ViewWordleElement>>) {
    for mut visibility in elements.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

pub fn hide_wordle_elements(mut elements: Query<&mut Visibility, With<ViewWordleElement>>) {
    for mut visibility in elements.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}
