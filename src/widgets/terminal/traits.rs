use ratatui::buffer::Cell as BufferCell;

pub trait Cell {
    fn apply(&self, cell: &mut BufferCell);
}

pub trait Screen {
    type Cell: Cell;

    fn cell(&self, row: u16, col: u16) -> Option<&Self::Cell>;
    fn size(&self) -> (u16, u16);
    fn cursor_position(&self) -> (u16, u16);
    fn hide_cursor(&self) -> bool;
    fn scrollback(&self) -> usize;
}
