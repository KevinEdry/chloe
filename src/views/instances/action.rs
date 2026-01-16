use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalAction {
    JumpToInstance(Uuid),
    SendInput { instance_id: Uuid, data: Vec<u8> },
    Scroll { instance_id: Uuid, delta: isize },
    ScrollToTop(Uuid),
    ScrollToBottom(Uuid),
}
