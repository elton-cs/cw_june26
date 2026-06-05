use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ViewWordleElement;

#[derive(Component)]
pub struct WordleTile;

const WORDLE_WORD: &str = "JESUS";
const WORDLE_TILE_SIZE: f32 = 80.;
const WORDLE_TILE_SPACING: f32 = WORDLE_TILE_SIZE * 1.1;

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
