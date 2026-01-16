use crate::app::App;
use crate::events::AppEvent;
use crate::events::dispatch;
use crate::views;
use crossterm::event::{Event, EventStream};
use futures::StreamExt;
use std::io;
use std::time::Duration;
use tokio::sync::mpsc;

const TICK_INTERVAL_MS: u64 = 100;

pub struct EventLoop {
    app_event_receiver: mpsc::UnboundedReceiver<AppEvent>,
    app_event_sender: mpsc::UnboundedSender<AppEvent>,
}

impl EventLoop {
    #[must_use]
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self {
            app_event_receiver: receiver,
            app_event_sender: sender,
        }
    }

    #[must_use]
    pub fn event_sender(&self) -> mpsc::UnboundedSender<AppEvent> {
        self.app_event_sender.clone()
    }

    #[allow(clippy::future_not_send)]
    pub async fn run<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut ratatui::Terminal<B>,
        app: &mut App,
    ) -> io::Result<()>
    where
        io::Error: From<B::Error>,
    {
        let mut event_stream = EventStream::new();
        let mut tick_interval = tokio::time::interval(Duration::from_millis(TICK_INTERVAL_MS));

        loop {
            terminal.draw(|frame| views::render(frame, app))?;

            tokio::select! {
                biased;

                maybe_crossterm_event = event_stream.next() => {
                    if let Some(Ok(Event::Key(key))) = maybe_crossterm_event {
                        let should_exit = dispatch::handle_key_event(app, key);
                        if should_exit {
                            return Ok(());
                        }
                    }
                }

                Some(app_event) = self.app_event_receiver.recv() => {
                    dispatch::handle_app_event(app, app_event);
                }

                _ = tick_interval.tick() => {
                    dispatch::handle_tick(app);
                }
            }
        }
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new()
    }
}
