use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct ViewWordleElement;

#[derive(Component)]
pub struct WordleTile;

const WORDLE_WORD: &str = "JESUS";
const WORDLE_TILE_SIZE: f32 = 80.;
const WORDLE_TILE_SPACING: f32 = WORDLE_TILE_SIZE * 1.1;

const DISPLAY_TILE_COUNT: usize = 26;
const DISPLAY_COLUMNS: usize = 2;
const DISPLAY_ROWS: usize = 13;
const DISPLAY_TILE_SIZE: f32 = 44.;
const DISPLAY_TILE_SPACING: f32 = DISPLAY_TILE_SIZE * 1.15;
const DISPLAY_PADDING: f32 = 18.;
const DISPLAY_EDGE_MARGIN: f32 = 16.;

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
                    WordleTile,
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
    let box_width = (DISPLAY_COLUMNS as f32 - 1.0) * DISPLAY_TILE_SPACING
        + DISPLAY_TILE_SIZE
        + DISPLAY_PADDING * 2.0;
    let box_height = (DISPLAY_ROWS as f32 - 1.0) * DISPLAY_TILE_SPACING
        + DISPLAY_TILE_SIZE
        + DISPLAY_PADDING * 2.0;
    let start_x = -((DISPLAY_COLUMNS as f32 - 1.0) * DISPLAY_TILE_SPACING) / 2.0;
    let start_y = ((DISPLAY_ROWS as f32 - 1.0) * DISPLAY_TILE_SPACING) / 2.0;
    let window_width = windows
        .single()
        .map(|window| window.width())
        .unwrap_or(1280.0);
    let display_x = -window_width / 2.0 + box_width / 2.0 + DISPLAY_EDGE_MARGIN;
    let letter_font = TextFont::from_font_size(DISPLAY_TILE_SIZE / 2.0);
    let number_font = TextFont::from_font_size(DISPLAY_TILE_SIZE / 4.0);

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

            for index in 0..DISPLAY_TILE_COUNT {
                let column = index / DISPLAY_ROWS;
                let row = index % DISPLAY_ROWS;
                let x = start_x + column as f32 * DISPLAY_TILE_SPACING;
                let y = start_y - row as f32 * DISPLAY_TILE_SPACING;
                let letter = (b'A' + index as u8) as char;

                p1.spawn((
                    WordleTile,
                    Transform::from_xyz(x, y, 1.0),
                    Visibility::default(),
                ))
                .with_children(|p2| {
                    p2.spawn((
                        Sprite::from_color(
                            PURPLE_300,
                            Vec2::new(DISPLAY_TILE_SIZE, DISPLAY_TILE_SIZE),
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
                        Transform::from_xyz(0.0, -DISPLAY_TILE_SIZE / 2.0 + 7.0, 2.0),
                    ));
                });
            }
        });
}

pub fn spawn_wordle_rune_display(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let box_width = (DISPLAY_COLUMNS as f32 - 1.0) * DISPLAY_TILE_SPACING
        + DISPLAY_TILE_SIZE
        + DISPLAY_PADDING * 2.0;
    let box_height = (DISPLAY_ROWS as f32 - 1.0) * DISPLAY_TILE_SPACING
        + DISPLAY_TILE_SIZE
        + DISPLAY_PADDING * 2.0;
    let start_x = -((DISPLAY_COLUMNS as f32 - 1.0) * DISPLAY_TILE_SPACING) / 2.0;
    let start_y = ((DISPLAY_ROWS as f32 - 1.0) * DISPLAY_TILE_SPACING) / 2.0;
    let window_width = windows
        .single()
        .map(|window| window.width())
        .unwrap_or(1280.0);
    let display_x = window_width / 2.0 - box_width / 2.0 - DISPLAY_EDGE_MARGIN;
    let letter_font = TextFont::from_font_size(DISPLAY_TILE_SIZE / 2.0);
    let number_font = TextFont::from_font_size(DISPLAY_TILE_SIZE / 4.0);

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

            for index in 0..DISPLAY_TILE_COUNT {
                let column = index / DISPLAY_ROWS;
                let row = index % DISPLAY_ROWS;
                let x = start_x + column as f32 * DISPLAY_TILE_SPACING;
                let y = start_y - row as f32 * DISPLAY_TILE_SPACING;
                let letter = (b'A' + index as u8) as char;

                p1.spawn((
                    WordleTile,
                    Transform::from_xyz(x, y, 1.0),
                    Visibility::default(),
                ))
                .with_children(|p2| {
                    p2.spawn((
                        Sprite::from_color(
                            BLUE_300,
                            Vec2::new(DISPLAY_TILE_SIZE, DISPLAY_TILE_SIZE),
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
                        Transform::from_xyz(0.0, -DISPLAY_TILE_SIZE / 2.0 + 7.0, 2.0),
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
