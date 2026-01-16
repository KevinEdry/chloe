#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorktreeAction {
    OpenInIde(usize),
    OpenInTerminal(usize),
}
