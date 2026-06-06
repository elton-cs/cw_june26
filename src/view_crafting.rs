use crate::constants::{
    VIEW_CONTENT_Z, VIEW_CRAFTING_BOX_PADDING, VIEW_CRAFTING_TILE_COLOR, VIEW_CRAFTING_TILE_COUNT,
    VIEW_CRAFTING_TILE_SIZE, VIEW_CRAFTING_TILE_SPACING, VIEW_FRAG_RUNE_COLUMNS,
    VIEW_FRAG_RUNE_DEFAULT_WINDOW_WIDTH, VIEW_FRAG_RUNE_EDGE_MARGIN,
    VIEW_FRAG_RUNE_NUMBER_BOTTOM_OFFSET, VIEW_FRAG_RUNE_PADDING, VIEW_FRAG_RUNE_ROWS,
    VIEW_FRAG_RUNE_TILE_COUNT, VIEW_FRAG_RUNE_TILE_SIZE, VIEW_FRAG_RUNE_TILE_SPACING,
    VIEW_FRAGMENT_TILE_COLOR, VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z, VIEW_OVERLAY_Z,
    VIEW_PANEL_COLOR, VIEW_RUNE_TILE_COLOR, VIEW_TEXT_COLOR,
};
use crate::view_ty::{DisplaySide, DisplayTileColor, ViewCraftingElement, ViewTile};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_crafting_frag_display(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    spawn_crafting_alphabet_display(
        &mut commands,
        &windows,
        DisplayTileColor::Purple,
        DisplaySide::Left,
    );
}

pub fn spawn_crafting_rune_display(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    spawn_crafting_alphabet_display(
        &mut commands,
        &windows,
        DisplayTileColor::Blue,
        DisplaySide::Right,
    );
}

pub fn spawn_crafting_box(mut commands: Commands) {
    let tile_positions = [
        (-1.0, 1.0),
        (1.0, 1.0),
        (0.0, 0.0),
        (-1.0, -1.0),
        (1.0, -1.0),
    ];
    debug_assert_eq!(tile_positions.len(), VIEW_CRAFTING_TILE_COUNT);

    let max_offset = tile_positions
        .iter()
        .map(|(x, y)| f32::max(f32::abs(*x), f32::abs(*y)))
        .fold(0.0_f32, |acc, value| acc.max(value));

    let box_width = (max_offset * 2.0 * VIEW_CRAFTING_TILE_SPACING)
        + VIEW_CRAFTING_TILE_SIZE
        + VIEW_CRAFTING_BOX_PADDING * 2.0;
    let box_height = (max_offset * 2.0 * VIEW_CRAFTING_TILE_SPACING)
        + VIEW_CRAFTING_TILE_SIZE
        + VIEW_CRAFTING_BOX_PADDING * 2.0;

    commands
        .spawn((
            ViewCraftingElement,
            Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z),
            Visibility::Hidden,
        ))
        .with_children(|p1| {
            p1.spawn((
                Sprite::from_color(VIEW_PANEL_COLOR, Vec2::new(box_width, box_height)),
                Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z),
            ));

            for (x_offset, y_offset) in tile_positions {
                p1.spawn((
                    ViewTile,
                    Transform::from_xyz(
                        x_offset * VIEW_CRAFTING_TILE_SPACING,
                        y_offset * VIEW_CRAFTING_TILE_SPACING,
                        VIEW_CONTENT_Z,
                    ),
                    Visibility::default(),
                ))
                .with_children(|p2| {
                    p2.spawn((
                        Sprite::from_color(
                            Color::from(VIEW_CRAFTING_TILE_COLOR),
                            Vec2::new(VIEW_CRAFTING_TILE_SIZE, VIEW_CRAFTING_TILE_SIZE),
                        ),
                        Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z),
                    ));
                });
            }
        });
}

pub fn show_crafting_elements(mut elements: Query<&mut Visibility, With<ViewCraftingElement>>) {
    for mut visibility in elements.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

pub fn hide_crafting_elements(mut elements: Query<&mut Visibility, With<ViewCraftingElement>>) {
    for mut visibility in elements.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn spawn_crafting_alphabet_display(
    commands: &mut Commands,
    windows: &Query<&Window, With<PrimaryWindow>>,
    tile_color: DisplayTileColor,
    side: DisplaySide,
) {
    let box_width = (VIEW_FRAG_RUNE_COLUMNS as f32 - 1.0) * VIEW_FRAG_RUNE_TILE_SPACING
        + VIEW_FRAG_RUNE_TILE_SIZE
        + VIEW_FRAG_RUNE_PADDING * 2.0;
    let box_height = (VIEW_FRAG_RUNE_ROWS as f32 - 1.0) * VIEW_FRAG_RUNE_TILE_SPACING
        + VIEW_FRAG_RUNE_TILE_SIZE
        + VIEW_FRAG_RUNE_PADDING * 2.0;
    let start_x = -((VIEW_FRAG_RUNE_COLUMNS as f32 - 1.0) * VIEW_FRAG_RUNE_TILE_SPACING) / 2.0;
    let start_y = ((VIEW_FRAG_RUNE_ROWS as f32 - 1.0) * VIEW_FRAG_RUNE_TILE_SPACING) / 2.0;
    let window_width = windows
        .single()
        .map(|window| window.width())
        .unwrap_or(VIEW_FRAG_RUNE_DEFAULT_WINDOW_WIDTH);
    let display_x = match side {
        DisplaySide::Left => -window_width / 2.0 + box_width / 2.0 + VIEW_FRAG_RUNE_EDGE_MARGIN,
        DisplaySide::Right => window_width / 2.0 - box_width / 2.0 - VIEW_FRAG_RUNE_EDGE_MARGIN,
    };
    let letter_font = TextFont::from_font_size(VIEW_FRAG_RUNE_TILE_SIZE / 2.0);
    let number_font = TextFont::from_font_size(VIEW_FRAG_RUNE_TILE_SIZE / 4.0);
    let tile_color = match tile_color {
        DisplayTileColor::Purple => VIEW_FRAGMENT_TILE_COLOR,
        DisplayTileColor::Blue => VIEW_RUNE_TILE_COLOR,
    };

    commands
        .spawn((
            ViewCraftingElement,
            Transform::from_xyz(display_x, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z),
            Visibility::Hidden,
        ))
        .with_children(|p1| {
            p1.spawn((
                Sprite::from_color(VIEW_PANEL_COLOR, Vec2::new(box_width, box_height)),
                Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z),
            ));

            for index in 0..VIEW_FRAG_RUNE_TILE_COUNT {
                let column = index / VIEW_FRAG_RUNE_ROWS;
                let row = index % VIEW_FRAG_RUNE_ROWS;
                let x = start_x + column as f32 * VIEW_FRAG_RUNE_TILE_SPACING;
                let y = start_y - row as f32 * VIEW_FRAG_RUNE_TILE_SPACING;
                let letter = (b'A' + index as u8) as char;

                p1.spawn((
                    ViewTile,
                    Transform::from_xyz(x, y, VIEW_CONTENT_Z),
                    Visibility::default(),
                ))
                .with_children(|p2| {
                    p2.spawn((
                        Sprite::from_color(
                            Color::from(tile_color),
                            Vec2::new(VIEW_FRAG_RUNE_TILE_SIZE, VIEW_FRAG_RUNE_TILE_SIZE),
                        ),
                        Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_ORIGIN_Z),
                    ));
                    p2.spawn((
                        Text2d::new(letter.to_string()),
                        TextColor(VIEW_TEXT_COLOR.into()),
                        letter_font.clone(),
                        Transform::from_xyz(VIEW_ORIGIN_X, VIEW_ORIGIN_Y, VIEW_CONTENT_Z),
                    ));
                    p2.spawn((
                        Text2d::new("100"),
                        TextColor(VIEW_TEXT_COLOR.into()),
                        number_font.clone(),
                        Transform::from_xyz(
                            VIEW_ORIGIN_X,
                            -VIEW_FRAG_RUNE_TILE_SIZE / 2.0 + VIEW_FRAG_RUNE_NUMBER_BOTTOM_OFFSET,
                            VIEW_OVERLAY_Z,
                        ),
                    ));
                });
            }
        });
}
