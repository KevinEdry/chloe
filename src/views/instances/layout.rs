use super::LayoutMode;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

#[must_use]
pub fn calculate_pane_areas(area: Rect, layout_mode: LayoutMode, pane_count: usize) -> Vec<Rect> {
    if pane_count == 0 {
        return Vec::new();
    }

    match layout_mode {
        LayoutMode::Single => vec![area],
        LayoutMode::HorizontalSplit => horizontal_split(area, pane_count),
        LayoutMode::VerticalSplit => vertical_split(area, pane_count),
        LayoutMode::Grid => grid_layout(area, pane_count),
    }
}

fn horizontal_split(area: Rect, pane_count: usize) -> Vec<Rect> {
    if pane_count == 0 {
        return Vec::new();
    }

    let pane_count_u32 = u32::try_from(pane_count).unwrap_or(u32::MAX);
    let constraints: Vec<Constraint> = (0..pane_count)
        .map(|_| Constraint::Ratio(1, pane_count_u32))
        .collect();

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area)
        .to_vec()
}

fn vertical_split(area: Rect, pane_count: usize) -> Vec<Rect> {
    if pane_count == 0 {
        return Vec::new();
    }

    let pane_count_u32 = u32::try_from(pane_count).unwrap_or(u32::MAX);
    let constraints: Vec<Constraint> = (0..pane_count)
        .map(|_| Constraint::Ratio(1, pane_count_u32))
        .collect();

    Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area)
        .to_vec()
}

fn grid_layout(area: Rect, pane_count: usize) -> Vec<Rect> {
    if pane_count == 0 {
        return Vec::new();
    }

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    let grid_size = ((pane_count as f64).sqrt().ceil() as usize).max(1);
    let rows = pane_count.div_ceil(grid_size);

    let rows_u32 = u32::try_from(rows).unwrap_or(u32::MAX);
    let row_constraints: Vec<Constraint> =
        (0..rows).map(|_| Constraint::Ratio(1, rows_u32)).collect();

    let row_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(area);

    let mut result = Vec::new();
    let mut remaining_panes = pane_count;

    for row_area in row_areas.iter() {
        let columns_in_row = remaining_panes.min(grid_size);
        let columns_in_row_u32 = u32::try_from(columns_in_row).unwrap_or(u32::MAX);
        let column_constraints: Vec<Constraint> = (0..columns_in_row)
            .map(|_| Constraint::Ratio(1, columns_in_row_u32))
            .collect();

        let column_areas = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(column_constraints)
            .split(*row_area);

        result.extend(column_areas.iter().take(columns_in_row));
        remaining_panes -= columns_in_row;

        if remaining_panes == 0 {
            break;
        }
    }

    result
}
