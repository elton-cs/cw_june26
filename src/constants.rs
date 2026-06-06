use bevy::color::{
    Srgba,
    palettes::{
        css::{BLACK, WHITE},
        tailwind::*,
    },
};

// Shared view colors
pub const VIEW_BACKGROUND_COLOR: Srgba = SLATE_100;
pub const VIEW_TEXT_COLOR: Srgba = BLACK;
pub const VIEW_LOGIN_PLACEHOLDER_TEXT_COLOR: Srgba = WHITE;
pub const VIEW_TITLE_TILE_COLOR: Srgba = SLATE_400;
pub const VIEW_PANEL_COLOR: Srgba = SLATE_200;
pub const VIEW_BUTTON_COLOR: Srgba = SLATE_200;
pub const VIEW_INPUT_FIELD_COLOR: Srgba = SLATE_200;
pub const VIEW_CROSSWORD_TILE_COLOR: Srgba = SLATE_300;
pub const VIEW_WORDLE_TILE_COLOR: Srgba = GREEN_400;
pub const VIEW_FRAGMENT_TILE_COLOR: Srgba = PURPLE_300;
pub const VIEW_RUNE_TILE_COLOR: Srgba = BLUE_300;

// Shared view layout
pub const VIEW_ORIGIN_X: f32 = 0.0;
pub const VIEW_ORIGIN_Y: f32 = 0.0;
pub const VIEW_ORIGIN_Z: f32 = 0.0;
pub const VIEW_CONTENT_Z: f32 = 1.0;
pub const VIEW_OVERLAY_Z: f32 = 2.0;

// Title layout
pub const VIEW_TITLE: &str = "CROSSWORDLE";
pub const VIEW_TITLE_Y: f32 = VIEW_CRAFTING_TILE_SPACING;
pub const VIEW_TITLE_TILE_SIZE: f32 = 80.;
pub const VIEW_TITLE_TILE_SPACING: f32 = VIEW_TITLE_TILE_SIZE * 1.1;

// Wordle layout
pub const VIEW_WORDLE_WORD: &str = "JESUS";
pub const VIEW_WORDLE_TILE_SIZE: f32 = 80.;
pub const VIEW_WORDLE_TILE_SPACING: f32 = VIEW_WORDLE_TILE_SIZE * 1.1;
pub const VIEW_WORDLE_BOX_PADDING: f32 = 18.;

// Fragment/rune display layout
pub const VIEW_FRAG_RUNE_TILE_COUNT: usize = 26;
pub const VIEW_FRAG_RUNE_COLUMNS: usize = 2;
pub const VIEW_FRAG_RUNE_ROWS: usize = 13;
pub const VIEW_FRAG_RUNE_TILE_SIZE: f32 = 44.;
pub const VIEW_FRAG_RUNE_TILE_SPACING: f32 = VIEW_FRAG_RUNE_TILE_SIZE * 1.1;
pub const VIEW_FRAG_RUNE_PADDING: f32 = 18.;
pub const VIEW_FRAG_RUNE_EDGE_MARGIN: f32 = 16.;
pub const VIEW_FRAG_RUNE_DEFAULT_WINDOW_WIDTH: f32 = 1280.0;
pub const VIEW_FRAG_RUNE_NUMBER_BOTTOM_OFFSET: f32 = 7.0;

// Crafting layout
pub const VIEW_CRAFTING_TILE_COUNT: usize = 5;
pub const VIEW_CRAFTING_TILE_SIZE: f32 = VIEW_WORDLE_TILE_SIZE;
pub const VIEW_CRAFTING_TILE_SPACING: f32 = VIEW_WORDLE_TILE_SPACING;
pub const VIEW_CRAFTING_BOX_PADDING: f32 = 18.;
pub const VIEW_CRAFTING_TILE_COLOR: Srgba = SLATE_300;

// Crossword layout
pub const VIEW_CROSSWORD_GRID_TILES_X: usize = 9;
pub const VIEW_CROSSWORD_GRID_TILES_Y: usize = 7;

pub const VIEW_CROSSWORD_TILE_SIZE: f32 = 80.;
pub const VIEW_CROSSWORD_TILE_SPACING: f32 = VIEW_CROSSWORD_TILE_SIZE * 1.1;
pub const VIEW_CROSSWORD_BOX_PADDING: f32 = 18.;

// Home layout
pub const VIEW_HOME_BUTTON_WIDTH: f32 = 200.0;
pub const VIEW_HOME_BUTTON_HEIGHT: f32 = 60.0;
pub const VIEW_HOME_BUTTON_Y_START: f32 = -140.0;
pub const VIEW_HOME_BUTTON_Y_SPACING: f32 = 80.0;
pub const VIEW_HOME_BUTTON_FONT_SIZE: f32 = 28.;

// Login layout
pub const VIEW_LOGIN_BUTTON_WIDTH: f32 = 200.0;
pub const VIEW_LOGIN_BUTTON_HEIGHT: f32 = 60.0;
pub const VIEW_LOGIN_INPUT_WIDTH: f32 = 600.0;
pub const VIEW_LOGIN_INPUT_HEIGHT: f32 = 60.0;
pub const VIEW_LOGIN_INPUT_Y_START: f32 = -140.0;
pub const VIEW_LOGIN_ELEMENT_Y_SPACING: f32 = 80.0;
pub const VIEW_LOGIN_BUTTON_FONT_SIZE: f32 = 28.;
