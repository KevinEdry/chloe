use super::colors::convert_alacritty_color;
use super::traits::{Cell, Screen};
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::index::{Column, Line, Point};
use alacritty_terminal::term::Term;
use alacritty_terminal::term::cell::Cell as AlacrittyCell;
use alacritty_terminal::term::cell::Flags;
use ratatui::{buffer::Cell as BufferCell, style::Modifier};

impl Cell for AlacrittyCell {
    fn apply(&self, buffer_cell: &mut BufferCell) {
        buffer_cell.set_char(self.c);
        buffer_cell.set_fg(convert_alacritty_color(self.fg));
        buffer_cell.set_bg(convert_alacritty_color(self.bg));

        let mut modifier = Modifier::empty();
        if self.flags.contains(Flags::BOLD) {
            modifier |= Modifier::BOLD;
        }
        if self.flags.contains(Flags::ITALIC) {
            modifier |= Modifier::ITALIC;
        }
        if self.flags.contains(Flags::UNDERLINE) {
            modifier |= Modifier::UNDERLINED;
        }
        if self.flags.contains(Flags::INVERSE) {
            modifier |= Modifier::REVERSED;
        }
        buffer_cell.set_style(buffer_cell.style().add_modifier(modifier));
    }
}

pub struct AlacrittyScreen<'a, T> {
    term: &'a Term<T>,
}

impl<'a, T> AlacrittyScreen<'a, T> {
    pub const fn new(term: &'a Term<T>) -> Self {
        Self { term }
    }
}

impl<T> Screen for AlacrittyScreen<'_, T> {
    type Cell = AlacrittyCell;

    fn cell(&self, row: u16, column: u16, scroll_offset: usize) -> Option<&Self::Cell> {
        let grid = self.term.grid();
        let adjusted_line = i32::from(row) - i32::try_from(scroll_offset).unwrap_or(i32::MAX);
        let point = Point {
            line: Line(adjusted_line),
            column: Column(usize::from(column)),
        };
        Some(&grid[point])
    }

    fn size(&self) -> (u16, u16) {
        let grid = self.term.grid();
        #[allow(clippy::cast_possible_truncation)]
        let rows = grid.screen_lines() as u16;
        #[allow(clippy::cast_possible_truncation)]
        let columns = grid.columns() as u16;
        (rows, columns)
    }

    fn cursor_position(&self) -> (u16, u16) {
        let cursor = self.term.grid().cursor.point;
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let row = cursor.line.0 as u16;
        #[allow(clippy::cast_possible_truncation)]
        let column = cursor.column.0 as u16;
        (row, column)
    }

    fn hide_cursor(&self) -> bool {
        !self
            .term
            .mode()
            .contains(alacritty_terminal::term::TermMode::SHOW_CURSOR)
    }

    fn scrollback(&self) -> usize {
        self.term.grid().history_size()
    }
}
