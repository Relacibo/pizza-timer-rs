use chrono::Duration;
use iced::widget::{row, text, Row};
use tokio::time::Instant;

use super::{setup::SetupView, AppMessage};

#[derive(Clone, Debug, Copy)]
pub struct RunningView {
    pub instant: Instant,
    pub instant_finish: Instant,
    pub duration_start: Duration,
    pub finished: bool,
}

impl RunningView {
    pub fn init(setup: SetupView, instant: Instant) -> RunningView {
        let SetupView {
            minutes, seconds, ..
        } = setup;
        let duration_start = Duration::try_minutes(minutes as i64).unwrap()
            + Duration::try_seconds(seconds as i64).unwrap();
        let instant_finish = instant + duration_start.to_std().unwrap();
        RunningView {
            duration_start,
            instant_finish,
            instant,
            finished: false,
        }
    }
    pub fn update(self, instant: Instant) -> RunningView {
        let finished = self.instant_finish <= instant;
        RunningView {
            instant,
            finished,
            ..self
        }
    }
    pub fn view(&self) -> Row<AppMessage> {
        let RunningView {
            instant,
            instant_finish,
            duration_start,
            ..
        } = self;
        let duration = *instant_finish - *instant;
        let duration = Duration::try_seconds(duration.as_secs() as i64).unwrap()
            + Duration::try_seconds(1).unwrap();
        let minutes = duration.num_minutes();
        let seconds = duration.num_seconds() % 60;

        let label_minutes = text(format!("{minutes:0>2}"));

        let label_seconds = text(format!("{seconds:0>2}"));

        let label_colon = text(":");
        row![label_minutes, label_colon, label_seconds]
    }
}
