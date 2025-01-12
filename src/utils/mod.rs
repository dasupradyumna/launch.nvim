/*----------------------------------------- UTILITY ITEMS ----------------------------------------*/

pub(crate) mod notify;
pub(crate) mod serde;

pub(crate) fn get_float_position_size(size: u32, lines: u32, columns: u32) -> [u32; 4] {
    let width = columns * size / 100;
    let height = lines * size / 100;
    let col = (columns - width) / 2 - 2;
    let row = (lines - height) / 2 - 2;

    [row, col, width, height]
}
