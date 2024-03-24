use chrono::Duration;
use iced::widget::{row, text, Row};
use tokio::time::Instant;

use super::AppMessage;

#[derive(Debug, Clone, Copy)]
pub struct RingingView {
    pub instant_finish: Instant,
    pub finished: bool,
}

impl RingingView {
    pub fn init(instant: Instant) -> RingingView {
        RingingView {
            instant_finish: instant + Duration::try_seconds(10).unwrap().to_std().unwrap(),
            finished: false,
        }
    }
    pub fn update(self, instant: Instant) -> RingingView {
        let finished = self.instant_finish <= instant;
        RingingView {
            finished,
            ..self
        }
    }
    pub fn view(&self) -> Row<AppMessage> {
        let label = text("Alarm");
        row![label]
    }
}
