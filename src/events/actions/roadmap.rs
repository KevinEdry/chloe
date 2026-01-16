#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoadmapAction {
    ConvertToTask(usize),
    Generate,
}
