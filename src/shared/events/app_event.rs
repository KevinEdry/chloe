use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum AppEvent {
    PtyOutput { pane_id: Uuid, data: Vec<u8> },
    PtyExit { pane_id: Uuid },
}
