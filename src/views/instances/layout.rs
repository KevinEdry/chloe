use super::state::{PaneNode, SplitDirection};
use ratatui::layout::Rect;
use uuid::Uuid;

const MINIMUM_PANE_WIDTH: u16 = 40;
const MINIMUM_PANE_HEIGHT: u16 = 10;
const ASPECT_RATIO_THRESHOLD: f32 = 1.5;
const DEFAULT_SPLIT_RATIO: f32 = 0.5;

#[must_use]
pub fn calculate_pane_areas(area: Rect, root: &PaneNode) -> Vec<(Uuid, Rect)> {
    let mut result = Vec::new();
    calculate_areas_recursive(area, root, &mut result);
    result
}

fn calculate_areas_recursive(area: Rect, node: &PaneNode, result: &mut Vec<(Uuid, Rect)>) {
    match node {
        PaneNode::Leaf(pane) => {
            result.push((pane.id, area));
        }
        PaneNode::Split {
            direction,
            ratio,
            first,
            second,
        } => {
            let (first_area, second_area) = split_area(area, *direction, *ratio);
            calculate_areas_recursive(first_area, first, result);
            calculate_areas_recursive(second_area, second, result);
        }
    }
}

fn split_area(area: Rect, direction: SplitDirection, ratio: f32) -> (Rect, Rect) {
    match direction {
        SplitDirection::Horizontal => {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let first_width = (f32::from(area.width) * ratio) as u16;
            let second_width = area.width.saturating_sub(first_width);

            let first = Rect {
                x: area.x,
                y: area.y,
                width: first_width,
                height: area.height,
            };
            let second = Rect {
                x: area.x.saturating_add(first_width),
                y: area.y,
                width: second_width,
                height: area.height,
            };
            (first, second)
        }
        SplitDirection::Vertical => {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let first_height = (f32::from(area.height) * ratio) as u16;
            let second_height = area.height.saturating_sub(first_height);

            let first = Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: first_height,
            };
            let second = Rect {
                x: area.x,
                y: area.y.saturating_add(first_height),
                width: area.width,
                height: second_height,
            };
            (first, second)
        }
    }
}

#[must_use]
pub fn choose_split_direction(pane_area: Rect) -> Option<SplitDirection> {
    let can_split_horizontal = pane_area.width >= MINIMUM_PANE_WIDTH * 2;
    let can_split_vertical = pane_area.height >= MINIMUM_PANE_HEIGHT * 2;

    match (can_split_horizontal, can_split_vertical) {
        (false, false) => None,
        (true, false) => Some(SplitDirection::Horizontal),
        (false, true) => Some(SplitDirection::Vertical),
        (true, true) => {
            let aspect_ratio = f32::from(pane_area.width) / f32::from(pane_area.height);
            if aspect_ratio >= ASPECT_RATIO_THRESHOLD {
                Some(SplitDirection::Horizontal)
            } else {
                Some(SplitDirection::Vertical)
            }
        }
    }
}

#[must_use]
pub const fn default_split_ratio() -> f32 {
    DEFAULT_SPLIT_RATIO
}

#[must_use]
pub fn find_biggest_pane_id(pane_areas: &[(Uuid, Rect)]) -> Option<Uuid> {
    pane_areas
        .iter()
        .max_by_key(|(_, area)| u32::from(area.width) * u32::from(area.height))
        .map(|(id, _)| *id)
}
