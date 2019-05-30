fn cell_value(cell: &str) -> u32 {
    cell.chars().take(2).map(|c| c as u32).sum::<u32>() % 2
}

#[allow(dead_code)]
pub fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
    cell_value(cell1) == cell_value(cell2)
}
