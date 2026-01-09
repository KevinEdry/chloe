use super::cursor::Cursor;
use super::traits::{Cell, Screen};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget},
};

pub struct PseudoTerminal<'a, S: Screen> {
    screen: &'a S,
    block: Option<Block<'a>>,
    style: Style,
    cursor: Cursor,
    scroll_offset: usize,
    show_scrollbar: bool,
}

impl<'a, S: Screen> PseudoTerminal<'a, S> {
    pub fn new(screen: &'a S) -> Self {
        Self {
            screen,
            block: None,
            style: Style::default(),
            cursor: Cursor::default(),
            scroll_offset: 0,
            show_scrollbar: true,
        }
    }

    pub fn cursor(mut self, cursor: Cursor) -> Self {
        self.cursor = cursor;
        self
    }

    pub const fn scroll_offset(mut self, offset: usize) -> Self {
        self.scroll_offset = offset;
        self
    }
}

impl<S: Screen> Widget for PseudoTerminal<'_, S> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner_area = if let Some(block) = &self.block {
            let inner = block.inner(area);
            block.clone().render(area, buf);
            inner
        } else {
            area
        };

        buf.set_style(inner_area, self.style);

        let scrollback_len = self.screen.scrollback();
        let has_scrollback = scrollback_len > 0 && self.show_scrollbar;

        let terminal_area = if has_scrollback {
            Rect {
                width: inner_area.width.saturating_sub(1),
                ..inner_area
            }
        } else {
            inner_area
        };

        render_screen(self.screen, terminal_area, buf);

        let (screen_rows, _) = self.screen.size();
        let should_show_cursor =
            self.cursor.is_visible() && !self.screen.hide_cursor() && self.scroll_offset == 0;

        if should_show_cursor {
            render_cursor(self.screen, terminal_area, buf, &self.cursor);
        }

        if has_scrollback {
            render_scrollbar(
                inner_area,
                buf,
                scrollback_len,
                screen_rows as usize,
                self.scroll_offset,
            );
        }
    }
}

fn render_screen<S: Screen>(screen: &S, area: Rect, buf: &mut Buffer) {
    let (screen_rows, screen_cols) = screen.size();
    let max_rows = area.height.min(screen_rows);
    let max_cols = area.width.min(screen_cols);

    for row in 0..max_rows {
        for col in 0..max_cols {
            let Some(cell) = screen.cell(row, col) else {
                continue;
            };

            let buffer_x = area.x + col;
            let buffer_y = area.y + row;

            if buffer_x < area.x + area.width && buffer_y < area.y + area.height {
                let buffer_cell = &mut buf[(buffer_x, buffer_y)];
                cell.apply(buffer_cell);
            }
        }
    }
}

fn render_cursor<S: Screen>(screen: &S, area: Rect, buf: &mut Buffer, cursor: &Cursor) {
    let (cursor_row, cursor_col) = screen.cursor_position();

    let cursor_x = area.x + cursor_col;
    let cursor_y = area.y + cursor_row;

    let is_within_bounds = cursor_x < area.x + area.width && cursor_y < area.y + area.height;

    if !is_within_bounds {
        return;
    }

    let buffer_cell = &mut buf[(cursor_x, cursor_y)];
    buffer_cell.set_symbol(cursor.get_symbol());
    buffer_cell.set_style(cursor.get_style());
}

fn render_scrollbar(
    area: Rect,
    buf: &mut Buffer,
    scrollback_len: usize,
    screen_rows: usize,
    scroll_offset: usize,
) {
    let scrollbar_area = Rect {
        x: area.x + area.width.saturating_sub(1),
        y: area.y,
        width: 1,
        height: area.height,
    };

    let total_content = scrollback_len + screen_rows;
    let position = scrollback_len.saturating_sub(scroll_offset);

    let mut scrollbar_state = ScrollbarState::new(total_content)
        .position(position)
        .viewport_content_length(area.height as usize);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(None)
        .end_symbol(None);

    scrollbar.render(scrollbar_area, buf, &mut scrollbar_state);
}
