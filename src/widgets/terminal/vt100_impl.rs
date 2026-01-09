use super::colors::convert_vt100_color;
use super::traits::{Cell, Screen};
use ratatui::{buffer::Cell as BufferCell, style::Modifier};

impl Cell for vt100::Cell {
    fn apply(&self, cell: &mut BufferCell) {
        let contents = self.contents();
        if contents.is_empty() {
            cell.set_char(' ');
        } else {
            cell.set_symbol(&contents);
        }

        cell.set_fg(convert_vt100_color(self.fgcolor()));
        cell.set_bg(convert_vt100_color(self.bgcolor()));

        let mut modifier = Modifier::empty();
        if self.bold() {
            modifier |= Modifier::BOLD;
        }
        if self.italic() {
            modifier |= Modifier::ITALIC;
        }
        if self.underline() {
            modifier |= Modifier::UNDERLINED;
        }
        if self.inverse() {
            modifier |= Modifier::REVERSED;
        }
        cell.set_style(cell.style().add_modifier(modifier));
    }
}

impl Screen for vt100::Screen {
    type Cell = vt100::Cell;

    fn cell(&self, row: u16, col: u16) -> Option<&Self::Cell> {
        self.cell(row, col)
    }

    fn size(&self) -> (u16, u16) {
        self.size()
    }

    fn cursor_position(&self) -> (u16, u16) {
        self.cursor_position()
    }

    fn hide_cursor(&self) -> bool {
        self.hide_cursor()
    }

    fn scrollback(&self) -> usize {
        self.scrollback()
    }
}
