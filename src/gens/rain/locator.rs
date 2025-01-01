use bevy::prelude::Transform;

use super::resources::{GridCursor, RainGridRowSpec, RainState, XZ};

/// Calculate the virtual grid used for rain drops.
/// Calculate the values that don't frequently change.
pub fn calc_grid_infreq_values(rain_state: &RainState) -> RainGridRowSpec {
    let row_capacity = get_row_capacity(&rain_state.capacity);
    let grid_capacity = square_product_integer(&row_capacity);
    let row_length = get_row_length(&rain_state.spacing, &grid_capacity);
    let row_length_half = row_length / 2.;

    RainGridRowSpec {
        row_capacity,
        row_length_half,
    }
}

/// Finds the values for the grid based on the inputs of the current frame iteration.
/// This is the freq variant, it changes each iteration.
pub fn calc_grid_start(row_length_half: &f32, player_pos: &Transform) -> XZ {
    get_grid_start(row_length_half, player_pos)
}

/// Calculate the XZ position of a drip
pub fn calc_drip_pos(rain_state: &RainState, grid_start: &XZ, i: &i32) -> XZ {
    let cursor = get_grid_cursors(&rain_state.row_spec.row_capacity, i);
    let drip_pos = get_drip_position(&rain_state.spacing, grid_start, cursor);
    drip_pos
}

/// Finds and rounds the square root of the input and squares that value to get an integer.
/// This will give you an integer value with a square root that is also an integer.
pub fn square_product_integer(rd_square_root: &i32) -> i32 {
    // let rd_square_root = get_row_capacity(&product);
    (rd_square_root * rd_square_root) as i32
}

/// Find the capacity for a given row (number of rain drops).
/// This is done from the total capacity of the grid,
/// returning a rounded square root of that capacity.
pub fn get_row_capacity(grid_capacity: &i32) -> i32 {
    (*grid_capacity as f32).sqrt().ceil() as i32
}

/// Get the row length from the rain spacing and number of drops a row has.
pub fn get_row_length(grid_spacing: &f32, row_capacity: &i32) -> f32 {
    grid_spacing * *row_capacity as f32
}

/// Get the coordinate of the top left corner of the grid.
pub fn get_grid_start(row_length_half: &f32, player: &Transform) -> XZ {
    XZ {
        x: player.translation.x - row_length_half,
        z: player.translation.z - row_length_half,
    }
}

/// Get the position of a rain drop based on grid index.
pub fn get_drip_position(grid_spacing: &f32, rel: &XZ, c: GridCursor) -> XZ {
    XZ {
        x: rel.x + (c.col_i_f32() * grid_spacing),
        z: rel.z + (c.row_i_f32() * grid_spacing),
    }
}

/// Find the column and row indices for the given index and row capacity.
pub fn get_grid_cursors(row_capacity: &i32, i: &i32) -> GridCursor {
    GridCursor {
        row_i: ((i / row_capacity) as f32).floor() as i32,
        col_i: (i % row_capacity) as i32,
    }
}
